//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2465/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2465(t47837: f64, t10069: f64, t13731: f64, t137: f64, t14103: f64, t47480: f64, t9675: f64, t14099: f64, t2453: f64, t9676: f64, t14109: f64, t9680: f64, t9685: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47838 = 0.34697458558045176417e-2_f64 * t47837;
    let t47839 = t10069 * t13731;
    let t47844 = t47480 * t14103 * t137 * t9675;
    let t47845 = 0.69394917116090352834e-2_f64 * t47844;
    let t47856 = t2453 * t14099;
    let t47857 = t47856 * t9676;
    let t47858 = 0.34697458558045176417e-2_f64 * t47857;
    let t47860 = t9680 * t14109 * t9685;
    (t47838, t47839, t47845, t47856, t47858, t47860)
}
