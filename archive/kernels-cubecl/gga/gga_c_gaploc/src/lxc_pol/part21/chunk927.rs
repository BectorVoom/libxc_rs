//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 927/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk927<F: Float>(t10144: F, t2343: F, t2268: F, t3327: F, t6313: F, t2317: F, t2761: F, t6525: F, t2321: F, t8237: F, t9074: F, t123: F, t7887: F) -> (F, F, F, F, F, F, F, F) {
    let t10145 = t2343 * t10144;
    let t10147 = F::cast_from(0.56910013271352299198e-1_f64) * t2268 * t10145;
    let t10150 = F::cast_from(0.37940008847568199465e-1_f64) * t6313 * t3327;
    let t10160 = t2761 * t2317;
    let t10161 = t6525 * t10160;
    let t10162 = F::cast_from(0.11856252764865062333e-2_f64) * t10161;
    let t10163 = t8237 * t2321;
    let t10164 = t9074 * t10163;
    let t10165 = F::cast_from(0.11856252764865062333e-2_f64) * t10164;
    let t10166 = t7887 * t123;
    (t10145, t10147, t10150, t10160, t10162, t10163, t10165, t10166)
}
