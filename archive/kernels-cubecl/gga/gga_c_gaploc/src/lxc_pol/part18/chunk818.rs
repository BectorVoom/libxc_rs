//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 818/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk818<F: Float>(t1967: F, t7805: F, t7810: F, t2087: F, t2103: F, t2197: F, t2638: F, t2664: F, t2669: F, t5782: F, t6060: F, t7751: F, t7756: F, t7759: F, t7766: F, t7769: F, t7772: F, t7775: F, t7780: F, t7782: F, t7786: F, t7790: F, t7792: F, t7795: F, t7798: F, t7800: F, t7807: F, t833: F) -> (F, F) {
    let t7811 = t1967 * t7805;
    let t7812 = t7810 * t7811;
    let t7814 = F::cast_from(0.71500979903700853338e0_f64) * t2103 * t7751 + F::cast_from(0.23005755572352449806e2_f64) * t2197 * t2664 + F::cast_from(0.23005755572352449806e2_f64) * t833 * t7756 + F::cast_from(0.11502877786176224903e2_f64) * t833 * t7759 - F::cast_from(0.13803453343411469884e2_f64) * t5782 * t2669 - F::cast_from(0.13803453343411469884e2_f64) * t2087 * t7766 - F::cast_from(0.25025342966295298669e1_f64) * t2638 * t7769 + F::cast_from(0.42900587942220512003e1_f64) * t2103 * t7772 - F::cast_from(0.21450293971110256001e1_f64) * t6060 * t7775 + F::cast_from(0.29792074959875355558e-1_f64) * t7780 + F::cast_from(0.14896037479937677779e-1_f64) * t7782 - F::cast_from(0.44688112439813033337e-1_f64) * t7786 - F::cast_from(0.14896037479937677779e-1_f64) * t7790 - F::cast_from(0.29792074959875355558e-1_f64) * t7792 + F::cast_from(0.59584149919750711116e-1_f64) * t7795 - F::cast_from(0.13491029502305448961e0_f64) * t7798 + F::cast_from(0.25561950635947166452e0_f64) * t7800 + F::cast_from(0.38342925953920749676e0_f64) * t7807 - F::cast_from(0.38342925953920749676e0_f64) * t7812;
    (t7812, t7814)
}
