//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta814 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2869;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2870;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta814<F: Float>(t13655: F, t4354: F, t41811: F, t5695: F, t4471: F, t17488: F, t892: F, t914: F, t10771: F, t10811: F, t14271: F, t14328: F, t14460: F, t14466: F, t17547: F, t17554: F, t2861: F, t2862: F, t2880: F, t2886: F, t2905: F, t42154: F, t42226: F, t42228: F, t4437: F, t49263: F, t5742: F, t5759: F, t59941: F, t59958: F, t59961: F, t59962: F, t59966: F, t59968: F, t933: F, t951: F, t4359: F, t49486: F, t4400: F, t49269: F, t13727: F, t14379: F, t10661: F, t2793: F, t13520: F, t14389: F, t10655: F, t17507: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t59970, t59972, t59975, t59981, t59982) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2869::<F>(t13655, t4354, t41811, t5695, t4471, t17488, t892, t914, t10771, t10811, t14271, t14328, t14460, t14466, t17547, t17554, t2861, t2862, t2880, t2886, t2905, t42154, t42226, t42228, t4437, t49263, t5742, t5759, t59941, t59958, t59961, t59962, t59966, t59968, t933, t951);
        let (t60006, t60008, t60010, t60016, t60021, t60023) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2870::<F>(t4359, t49486, t4400, t49269, t13727, t14379, t10661, t2793, t5695, t13520, t14389, t10655, t17507);
    (t59970, t59972, t59975, t59981, t59982, t60006, t60008, t60010, t60016, t60021, t60023)
}
