//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1409/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1409<F: Float>(t109645: F, t9535: F, t42127: F, t539: F, t2331: F, t4348: F, t6204: F, t33940: F, t9511: F, t109516: F, t109929: F, t113717: F, t113749: F, t115073: F, t115077: F, t115080: F, t115085: F, t115090: F, t115094: F, t115099: F, t32354: F, t32439: F, t32443: F, t33827: F, t33937: F, t33941: F, t9539: F, t9864: F) -> (F, F) {
    let t115104 = t109645 * t9535;
    let t115105 = t539 * t42127;
    let t115108 = t6204 * t115105 * t2331 * t4348;
    let t115111 = t9511 * t33940;
    let t115114 = 0.77382407407407407407e-2 * t113717 + t115073 + 0.44675925925925925926e-3 * t109516 - t115077 - t115080 - 0.17361111111111111111e-2 * t109929 * t9864 - 0.10416666666666666667e-1 * t33941 * t32443 - 0.40208333333333333334e-2 * t115085 * t32443 - 0.23214722222222222222e-2 * t113749 + 0.81018518518518518518e-2 * t115090 - 0.116403125e-2 * t33937 * t115094 - 0.40208333333333333334e-2 * t32439 * t115099 - 0.69444444444444444444e-2 * t32354 * t33827 + 0.898632125e-3 * t115104 * t115108 - 0.34722222222222222222e-2 * t115111 * t9539;
    (t115108, t115114)
}
