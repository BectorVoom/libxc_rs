//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 572/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk572(t1356: f64, t1360: f64, t1387: f64, t1413: f64, t1418: f64, t1421: f64, t2052: f64, t2059: f64, t246: f64, t2896: f64, t2897: f64, t2997: f64, t2998: f64, t3128: f64, t3162: f64, t3165: f64, t765: f64) -> f64 {
    let t3170 = t2052 - t2059 + t1356 + 0.675260332e-1_f64 * t765 * t3162 + 0.1350520664e0_f64 * t765 * t3165 + t1360 - t2896 + t2897 + t2997 - 0.285764e-1_f64 * t246 * t3128 + t2998 + t1387 + t1413 - t1418 - t1421;
    t3170
}
