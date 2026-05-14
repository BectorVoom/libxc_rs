//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1429/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1429<F: Float>(t109518: F, t109626: F, t109856: F, t109858: F, t109875: F, t109888: F, t115017: F, t115054: F, t115108: F, t115304: F, t115661: F, t115663: F, t115669: F, t115676: F, t115679: F, t115684: F, t32439: F, t33784: F, t33937: F, t9516: F, t9536: F, t9864: F) -> (F,) {
    let t115690 = -0.34722222222222222222e-2 * t9536 * t115017 + t115661 + t115663 + 0.13402777777777777778e-2 * t109856 + 0.67013888888888888888e-3 * t109858 - 0.69444444444444444445e-2 * t109626 * t115669 - 0.41270617283950617284e-2 * t109875 + 0.20104166666666666667e-2 * t9516 * t115054 + t115676 + t115679 + 0.69841875000000000001e-2 * t33937 * t115108 - 0.120625e-1 * t32439 * t115304 + 0.3086419753086419753e-2 * t115684 - 0.16975308641975308642e-1 * t109888 * t9864 - 0.120625e-1 * t109518 * t33784;
    (t115690,)
}
