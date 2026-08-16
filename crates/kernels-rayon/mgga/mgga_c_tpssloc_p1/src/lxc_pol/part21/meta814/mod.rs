//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta814 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2869;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2870;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta814(t13655: f64, t4354: f64, t41811: f64, t5695: f64, t4471: f64, t17488: f64, t892: f64, t914: f64, t10771: f64, t10811: f64, t14271: f64, t14328: f64, t14460: f64, t14466: f64, t17547: f64, t17554: f64, t2861: f64, t2862: f64, t2880: f64, t2886: f64, t2905: f64, t42154: f64, t42226: f64, t42228: f64, t4437: f64, t49263: f64, t5742: f64, t5759: f64, t59941: f64, t59958: f64, t59961: f64, t59962: f64, t59966: f64, t59968: f64, t933: f64, t951: f64, t4359: f64, t49486: f64, t4400: f64, t49269: f64, t13727: f64, t14379: f64, t10661: f64, t2793: f64, t13520: f64, t14389: f64, t10655: f64, t17507: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t59970, t59972, t59975, t59981, t59982) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2869(t13655, t4354, t41811, t5695, t4471, t17488, t892, t914, t10771, t10811, t14271, t14328, t14460, t14466, t17547, t17554, t2861, t2862, t2880, t2886, t2905, t42154, t42226, t42228, t4437, t49263, t5742, t5759, t59941, t59958, t59961, t59962, t59966, t59968, t933, t951);
        let (t60006, t60008, t60010, t60016, t60021, t60023) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2870(t4359, t49486, t4400, t49269, t13727, t14379, t10661, t2793, t5695, t13520, t14389, t10655, t17507);
    (t59970, t59972, t59975, t59981, t59982, t60006, t60008, t60010, t60016, t60021, t60023)
}
