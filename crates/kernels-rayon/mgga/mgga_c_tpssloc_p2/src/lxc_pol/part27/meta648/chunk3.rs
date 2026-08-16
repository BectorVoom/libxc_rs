//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2241/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2241(t25555: f64, t82822: f64, t25529: f64, t6680: f64, t1920: f64, t2966: f64, t7614: f64, t14622: f64, t14651: f64, t1539: f64, t1610: f64, t23478: f64, t23633: f64, t23635: f64, t23685: f64, t23707: f64, t25567: f64, t25712: f64, t3200: f64, t4684: f64, t61774: f64, t6687: f64, t6784: f64, t6800: f64, t6811: f64, t7619: f64, t82566: f64, t82799: f64, t82806: f64) -> f64 {
    let t89421 = 0.18277045187202515961e-2_f64 * t82822 * t25555;
    let t89429 = 0.14621636149762012769e-1_f64 * t6680 * t25529;
    let t89431 = t1920 * t2966 * t7614;
    let t89433 = -2.0_f64 * t3200 * t25567 * t4684 + t82799 - 0.16449340668482264365e-1_f64 * t6687 * t25712 * t23478 * t23685 - t3200 * t7619 * t14622 + 0.26806332941230356743e-1_f64 * t82806 + 0.27415567780803773942e-2_f64 * t6687 * t6784 * t82566 * t1539 + t1610 * t23707 + t89421 + 0.54831135561607547884e-2_f64 * t23633 * t23635 * t61774 * t6800 + 2.0_f64 * t14651 * t6811 - t89429 - 0.18277045187202515961e-2_f64 * t89431;
    t89433
}
