//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1237/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1237<F: Float>(t16887: F, t1701: F, t5546: F, t104949: F, t104953: F, t104955: F, t104967: F, t104969: F, t2036: F, t26729: F, t30038: F, t4703: F, t5785: F, t5790: F, t94578: F, t94600: F, t94620: F) -> (F, F) {
    let t118882 = t1701 * t5546 * t16887;
    let t118893 = 0.45306850413028723348e0 * t5785 * t118882 + 0.45306850413028723348e0 * t26729 * t30038 - 0.29634667152263374488e-1 * t94578 - t94600 + 0.66678001092592592595e-1 * t104949 - t104953 + t104955 + t104967 - t104969 + 0.3704333394032921811e-2 * t94620 - 0.54738951849294959987e0 * t2036 * t5790 * t4703;
    (t118882, t118893)
}
