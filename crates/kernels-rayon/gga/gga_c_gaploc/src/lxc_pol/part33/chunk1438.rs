//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1438/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1438(t38974: f64, t550: f64, t2033: f64, t28827: f64, t28828: f64, t28833: f64, t28836: f64, t28839: f64, t28841: f64, t28851: f64, t28854: f64, t28859: f64, t28861: f64, t28864: f64, t28865: f64, t28873: f64, t33732: f64, t39044: f64, t549: f64, t7584: f64, t7585: f64) -> (f64, f64) {
    let t39272 = t550 * t38974;
    let t39281 = -t28827 + 0.38342925953920749677e0_f64 * t28828 + 0.38342925953920749677e0_f64 * t28833 - t28836 + t28839 + t28841 - t28851 - t28854 + t33732 + t28859 + 0.79445533226334281486e-1_f64 * t2033 * t549 * t39272 - 0.38342925953920749677e0_f64 * t28861 + t28864 - 0.76685851907841499354e0_f64 * t28865 + t28873 - 0.23005755572352449806e2_f64 * t7584 * t7585 * t39044;
    (t39272, t39281)
}
