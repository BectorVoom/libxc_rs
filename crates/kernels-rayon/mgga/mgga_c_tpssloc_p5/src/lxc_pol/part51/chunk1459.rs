//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1459/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1459(t31304: f64, t7754: f64, t33366: f64, t6876: f64, t24994: f64, t8606: f64, t24996: f64, t122678: f64, t122681: f64, t122685: f64, t122692: f64, t1266: f64, t1459: f64, t1869: f64, t2036: f64, t25958: f64, t26870: f64, t31246: f64, t31532: f64, t33133: f64, t33579: f64, t4037: f64, t7040: f64, t7171: f64, t7670: f64, t7943: f64) -> f64 {
    let t122696 = t31304 * t7754;
    let t122697 = t6876 * t33366;
    let t122698 = t8606 * t24994;
    let t122700 = 6.0_f64 * t122698 * t24996;
    let t122701 = -2.0_f64 * t122685 * t1459 - t1266 * t33579 - t1869 * t26870 - t2036 * t25958 - t31246 * t7943 - 2.0_f64 * t31532 * t4037 + 3.0_f64 * t33133 * t7171 - t7040 * t7670 + t122678 - t122681 - t122692 + t122696 - t122697 + t122700;
    t122701
}
