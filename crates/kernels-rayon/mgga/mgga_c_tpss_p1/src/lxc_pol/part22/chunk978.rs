//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 978/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk978(t10498: f64, t187: f64, t8043: f64, t1398: f64, t8096: f64, t8077: f64, t123: f64, t1354: f64, t2349: f64, t10470: f64, t10471: f64, t10472: f64, t1692: f64, t2133: f64, t2433: f64, t2439: f64, t3548: f64, t7929: f64, t7932: f64, t7936: f64, t7945: f64, t8000: f64, t8001: f64, t8019: f64, t8023: f64, t8029: f64, t8040: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10500 = 0.19751673498613801407e-1_f64 * t10498 * t187;
    let t10501 = 12.0_f64 * t8043;
    let t10502 = t1398 * t8096;
    let t10509 = 8.0_f64 * t8077;
    let t10510 = t1354 * t123;
    let t10511 = t10510 * t2349;
    let t10512 = 0.10843581300301739842e-1_f64 * t10511;
    let t10513 = 2.0_f64 * t10502 * t1692 * t2433 + 3.0_f64 * t2133 * t2439 * t3548 - t10470 + t10471 - t10472 + t10500 + t10501 + t10509 + t10512 + t7929 - t7932 - t7936 + t7945 + t8000 + t8001 - t8019 + t8023 - t8029 - t8040;
    (t10500, t10501, t10502, t10509, t10512, t10513)
}
