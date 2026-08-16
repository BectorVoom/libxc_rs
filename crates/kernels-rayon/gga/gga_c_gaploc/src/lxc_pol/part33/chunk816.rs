//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 816/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk816(t1967: f64, t7805: f64, t7810: f64, t2087: f64, t2103: f64, t2197: f64, t2638: f64, t2664: f64, t2669: f64, t5782: f64, t6060: f64, t7751: f64, t7756: f64, t7759: f64, t7766: f64, t7769: f64, t7772: f64, t7775: f64, t7780: f64, t7782: f64, t7786: f64, t7790: f64, t7792: f64, t7795: f64, t7798: f64, t7800: f64, t7807: f64, t833: f64) -> (f64, f64) {
    let t7811 = t1967 * t7805;
    let t7812 = t7810 * t7811;
    let t7814 = 0.71500979903700853338e0_f64 * t2103 * t7751 + 0.23005755572352449806e2_f64 * t2197 * t2664 + 0.23005755572352449806e2_f64 * t833 * t7756 + 0.11502877786176224903e2_f64 * t833 * t7759 - 0.13803453343411469884e2_f64 * t5782 * t2669 - 0.13803453343411469884e2_f64 * t2087 * t7766 - 0.25025342966295298669e1_f64 * t2638 * t7769 + 0.42900587942220512003e1_f64 * t2103 * t7772 - 0.21450293971110256001e1_f64 * t6060 * t7775 + 0.29792074959875355558e-1_f64 * t7780 + 0.14896037479937677779e-1_f64 * t7782 - 0.44688112439813033337e-1_f64 * t7786 - 0.14896037479937677779e-1_f64 * t7790 - 0.29792074959875355558e-1_f64 * t7792 + 0.59584149919750711116e-1_f64 * t7795 - 0.13491029502305448961e0_f64 * t7798 + 0.25561950635947166452e0_f64 * t7800 + 0.38342925953920749676e0_f64 * t7807 - 0.38342925953920749676e0_f64 * t7812;
    (t7812, t7814)
}
