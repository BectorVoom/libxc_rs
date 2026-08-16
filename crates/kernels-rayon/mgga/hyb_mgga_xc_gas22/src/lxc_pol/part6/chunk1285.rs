//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1285/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1285(t23699: f64, t7866: f64, t9839: f64, t10081: f64, t1181: f64, t1861: f64, t19: f64, t23043: f64, t23647: f64, t23649: f64, t23655: f64, t23746: f64, t23749: f64, t23756: f64, t23759: f64, t23762: f64, t23765: f64, t26: f64, t27038: f64, t27649: f64, t27732: f64, t2949: f64, t2950: f64, t2970: f64, t3: f64, t3114: f64, t3917: f64, t547: f64, t668: f64, t7835: f64, t7842: f64, t7852: f64, t7856: f64, t7868: f64, t8148: f64, t8201: f64, t9846: f64, t9851: f64) -> f64 {
    let t27815 = t7866 * t23699 * t9839;
    let t27843 = -7.0_f64 / 144.0_f64 * t7866 * t7868 * t27732 - 35.0_f64 / 216.0_f64 * t23647 * t23649 * t27649 - t2970 * t23043 * t9846 / 6.0_f64 - t2970 * t7835 * t7856 * t3 / 6.0_f64 + t7842 * t7852 * t9839 / 8.0_f64 - 7.0_f64 / 216.0_f64 * t27815 - t23746 / 32.0_f64 - t23749 / 16.0_f64 - 3.0_f64 / 32.0_f64 * t547 * t9851 - 3.0_f64 / 32.0_f64 * t19 * t26 * t10081 * t668 - 3.0_f64 / 64.0_f64 * t19 * t26 * t3917 * t1861 - 3.0_f64 / 32.0_f64 * t1181 * t8148 - 3.0_f64 / 16.0_f64 * t1181 * t8201 - 3.0_f64 / 8.0_f64 * t2949 * t2950 * t3114 - 7.0_f64 / 18.0_f64 * t7866 * t23655 * t27038 + t23756 / 72.0_f64 - t23759 / 48.0_f64 - t23762 / 48.0_f64 - t23765 / 96.0_f64;
    t27843
}
