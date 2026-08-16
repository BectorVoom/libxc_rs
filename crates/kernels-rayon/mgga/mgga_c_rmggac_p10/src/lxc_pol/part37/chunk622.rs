//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 622/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk622(t15624: f64, t515: f64, t1971: f64, t7230: f64, t14581: f64, t2344: f64, t14585: f64, t2329: f64, t14589: f64, t2333: f64, t15311: f64, t15315: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15625 = t515 * t15624;
    let t15626 = t1971 * t15625;
    let t15627 = t7230 * t15626;
    let t15628 = 0.53205749866622299248e-5_f64 * t15627;
    let t15632 = t14581 * t2344;
    let t15633 = 0.10227998120342003148e-1_f64 * t15632;
    let t15634 = t14585 * t2329;
    let t15635 = 0.13637330827122670864e-1_f64 * t15634;
    let t15636 = t14589 * t2333;
    let t15637 = 0.68186654135613354322e-2_f64 * t15636;
    let t15640 = 0.10227998120342003148e-1_f64 * t15311;
    let t15643 = 0.40911992481368012592e-1_f64 * t15315;
    (t15626, t15628, t15633, t15635, t15637, t15640, t15643)
}
