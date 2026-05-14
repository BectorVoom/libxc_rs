//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 957/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk957<F: Float>(t1083: F, t1089: F, t137: F, t4875: F, t598: F, t30721: F, t30725: F, t30729: F, t34743: F, t34746: F, t34747: F, t34749: F, t34751: F, t34754: F, t34757: F, t34762: F, t34767: F, t34769: F, t34771: F, t34775: F, t34779: F, t34783: F) -> (F,) {
    let t34788 = t598 * t1089 * t1083 * t137 * t4875;
    let t34790 = -t34743 - 0.18868855373762491241e-2 * t30721 - t34746 - 0.68598428988911579156e-2 * t34747 - 0.34299214494455789578e-2 * t34749 + 0.17149607247227894789e-2 * t34751 + t34754 + 0.64311027177104605458e-3 * t34757 + 0.31448092289604152068e-2 * t30725 + t30729 + 0.31448092289604152068e-3 * t34762 - 0.41930789719472202758e-3 * t34767 - 0.85748036236139473944e-3 * t34769 - 0.41930789719472202758e-3 * t34771 - 0.31448092289604152068e-3 * t34775 - 0.62896184579208304136e-3 * t34779 - 0.41930789719472202758e-3 * t34783 - 0.31448092289604152068e-3 * t34788;
    (t34790,)
}
