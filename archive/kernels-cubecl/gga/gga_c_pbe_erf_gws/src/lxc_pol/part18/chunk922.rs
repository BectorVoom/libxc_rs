//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 922/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk922<F: Float>(t5103: F, t2660: F, t3451: F, t1879: F, t4358: F, t532: F, t198: F, t186: F, t561: F, t2737: F, t2741: F, t2730: F, t3564: F) -> (F, F, F, F, F, F, F) {
    let t10307 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t5103;
    let t10309 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t2660 * t3451;
    let t10311 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t1879 * t3451;
    let t10313 = -t532 - F::cast_from(3.0_f64) * t4358;
    let t10314 = t198 * t10313;
    let t10315 = t186 * t10314;
    let t10317 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t561 * t10315;
    let t10319 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t2741 * t2737;
    let t10321 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t2730 * t3564;
    (t10307, t10309, t10311, t10313, t10317, t10319, t10321)
}
