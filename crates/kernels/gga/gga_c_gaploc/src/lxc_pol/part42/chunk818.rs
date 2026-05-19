//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 818/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk818<F: Float>(t13262: F, t6313: F, t13327: F, t13277: F, t11271: F, t2268: F, t2349: F, t11187: F, t2317: F, t6525: F, t11254: F, t2293: F) -> (F, F, F, F, F, F) {
    let t44437 = F::cast_from(0.7588001769513639893e-1_f64) * t6313 * t13262;
    let t44439 = F::cast_from(0.37940008847568199465e-1_f64) * t6313 * t13327;
    let t44443 = F::cast_from(0.22764005308540919679e0_f64) * t6313 * t13277;
    let t44457 = F::cast_from(0.85365019907028448797e-1_f64) * t2268 * t11271 * t2349;
    let t44468 = t6525 * t11187 * t2317;
    let t44469 = F::cast_from(0.11856252764865062333e-2_f64) * t44468;
    let t44470 = t11254 * t2293;
    (t44437, t44439, t44443, t44457, t44469, t44470)
}
