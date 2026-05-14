//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1241/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1241<F: Float>(t115495: F, t5838: F, t4710: F, t53: F, t5591: F, t72: F, t4698: F, t58: F, t22591: F, t554: F, t118968: F, t105201: F, t115492: F, t118934: F, t118938: F, t23774: F, t23825: F, t23842: F, t23847: F, t30091: F, t39847: F, t5579: F, t77125: F, t94722: F) -> (F, F, F) {
    let t118990 = t5838 * t115495;
    let t118994 = t5591 * t72 * t4710 * t53;
    let t118999 = t58 * t4698;
    let t119001 = t22591 * t118999 * t554;
    let t119012 = t22591 * t118968 * t554;
    let t119019 = -0.88904001456790123461e-1 * t5838 * t115492 + 0.11113000182098765433e-1 * t118990 - 0.24167761770734866964e0 * t23842 * t118994 + 0.24167761770734866964e0 * t23825 * t118994 - 0.45306850413028723348e0 * t23847 * t119001 - 0.22226000364197530866e-1 * t94722 - 0.90613700826057446696e0 * t105201 * t30091 - 0.90613700826057446696e0 * t23847 * t118934 - 0.90613700826057446696e0 * t23847 * t118938 + 0.13592055123908617004e1 * t39847 * t119012 - 0.60010200983333333334e0 * t23774 * t5579 * t72 * t77125;
    (t119001, t119012, t119019)
}
