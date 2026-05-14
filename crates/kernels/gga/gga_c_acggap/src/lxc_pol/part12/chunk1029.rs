//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1029/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1029<F: Float>(t34745: F, t34751: F, t34753: F, t30717: F, t30721: F, t30725: F, t32561: F, t34747: F, t34749: F, t34757: F, t34762: F, t34767: F, t34769: F, t34771: F, t34775: F, t34779: F, t34783: F, t34788: F) -> (F,) {
    let t37230 = 0.34299214494455789578e-2 * t34745;
    let t37233 = 0.34299214494455789578e-2 * t34751;
    let t37234 = 0.64025200389650807212e-1 * t34753;
    let t37245 = -35.0 / 54.0 * t30717 - 0.37737710747524982482e-2 * t30721 - t37230 - 0.13719685797782315831e-1 * t34747 - 0.68598428988911579156e-2 * t34749 + t37233 + t37234 + 0.12862205435420921092e-2 * t34757 + 0.62896184579208304137e-2 * t30725 + t32561 + 0.62896184579208304138e-3 * t34762 - 0.83861579438944405518e-3 * t34767 - 0.17149607247227894789e-2 * t34769 - 0.83861579438944405518e-3 * t34771 - 0.62896184579208304138e-3 * t34775 - 0.12579236915841660828e-2 * t34779 - 0.83861579438944405518e-3 * t34783 - 0.62896184579208304138e-3 * t34788;
    (t37245,)
}
