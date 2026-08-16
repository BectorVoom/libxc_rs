//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1043/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1043(t14451: f64, t1652: f64, t5148: f64, t570: f64, t71910: f64, t8940: f64, t72011: f64, t76292: f64, t76311: f64, t78017: f64, t78018: f64, t78019: f64, t78020: f64, t78021: f64, t78024: f64, t78027: f64, t78028: f64) -> f64 {
    let t78030 = t5148 * t14451 * t1652;
    let t78031 = 0.2993560425465952141e-1_f64 * t78030;
    let t78034 = 0.11974241701863808564e0_f64 * t8940 * t71910 * t570;
    let t78035 = -t78017 - t78018 - t78019 + t78020 + t78021 - t78024 - t78027 + t76292 - t78028 + t72011 + t78031 + t78034 + t76311;
    t78035
}
