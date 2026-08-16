//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 810/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk810(t255: f64, t571: f64, t8196: f64, t2086: f64, t980: f64, t2627: f64, t6518: f64, t2605: f64, t5100: f64, t1604: f64, t8071: f64, t6086: f64, t7624: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8198 = t571 * t8196 * t255;
    let t8201 = t980 * t2086;
    let t8224 = 0.76830240467580968652e0_f64 * t6518 * t2627;
    let t8227 = t5100 * t2605;
    let t8231 = 0.54878743191129263322e-2_f64 * t1604 * t8071;
    let t8232 = t6086 * t7624;
    (t8198, t8201, t8224, t8227, t8231, t8232)
}
