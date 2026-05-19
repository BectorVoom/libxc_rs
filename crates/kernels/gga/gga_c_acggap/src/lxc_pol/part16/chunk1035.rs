//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1035/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1035<F: Float>(t36388: F, t1967: F, t8566: F, t1998: F, t4557: F, t5351: F, t7948: F, t309: F, t556: F, t322: F, t29979: F, t620: F) -> (F, F, F, F, F) {
    let t36389 = F::cast_from(0.34299214494455789578e-2_f64) * t36388;
    let t36390 = t1967 * t8566;
    let t36391 = F::cast_from(0.37737710747524982482e-2_f64) * t36390;
    let t36392 = t1998 * t4557;
    let t36405 = t7948 * t5351;
    let t36416 = t556 * t309;
    let t36417 = t36416 * t322;
    let t36419 = t29979 * t620 * t36417;
    (t36389, t36391, t36392, t36405, t36419)
}
