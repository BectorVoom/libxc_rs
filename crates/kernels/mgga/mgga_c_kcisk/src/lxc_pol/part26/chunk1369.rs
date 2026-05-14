//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1369/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1369<F: Float>(t109171: F, t8189: F, t13917: F, t35003: F, t9536: F, t113584: F, t114982: F, t114983: F, t114991: F, t114997: F, t115001: F, t115004: F, t115162: F, t118689: F, t118704: F, t25413: F, t32339: F, t32436: F, t33914: F, t34950: F, t35004: F) -> (F, F) {
    let t120006 = 2.0 * t109171 * t8189;
    let t120010 = t9536 * t13917 * t35003;
    let t120023 = 0.61728395061728395061e-2 * t32339 * t35004 - 0.77160493827160493827e-3 * t120010 - t114982 + 0.23148148148148148148e-2 * t114983 + 0.77382407407407407407e-3 * t118689 - 0.69444444444444444444e-2 * t114991 - t114997 - 0.41270617283950617283e-2 * t113584 + 0.69644166666666666664e-2 * t118704 + 0.13888888888888888889e-1 * t9536 * t115162 * t33914 * t25413 + 0.34722222222222222222e-2 * t32436 * t34950 + t115001 + t115004;
    (t120006, t120023)
}
