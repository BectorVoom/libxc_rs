//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3257/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3257(t13783: f64, t13804: f64, t1410: f64, t1868: f64, t1883: f64, t21969: f64, t22016: f64, t22279: f64, t3934: f64, t4012: f64, t48509: f64, t48516: f64, t48518: f64, t48529: f64, t48532: f64, t48563: f64, t5591: f64, t5673: f64, t6816: f64, t73847: f64, t74232: f64, t74249: f64, t74257: f64, t828: f64, t85553: f64, t85741: f64, t85752: f64, t85764: f64, t85778: f64) -> f64 {
    let t85780 = -0.85748036236139473942e-4_f64 * t85741 - t48509 + 0.91464571985215438873e-3_f64 * t48516 + 0.11337795902333997111e0_f64 * t48518 + 0.91464571985215438872e-3_f64 * t48529 - t48532 - 0.22869001264178397701e-3_f64 * t74232 - 0.77173232612525526552e-2_f64 * t13804 * t5673 * t85553 * t22016 - 0.38115002106963996168e-4_f64 * t74249 + 0.24009450146119052705e-1_f64 * t85752 - 0.64311027177104605458e-3_f64 * t3934 * t5673 * t73847 * t1883 + 0.30492001685571196935e-3_f64 * t74257 - 0.60246173160355784831e-6_f64 * t48563 - 0.25724410870841842184e-1_f64 * t3934 * t13783 * t1883 * t22279 - 0.60023625365297631763e-1_f64 * t85764 + 0.12862205435420921092e-1_f64 * t1410 * t4012 * t828 * t21969 * t1868 + 0.12862205435420921092e-1_f64 * t1410 * t4012 * t828 * t6816 * t5591 + 0.7623000421392799234e-3_f64 * t85778;
    t85780
}
