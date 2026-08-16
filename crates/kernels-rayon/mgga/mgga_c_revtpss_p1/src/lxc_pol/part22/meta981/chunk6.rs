//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3317/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3317(t10073: f64, t18742: f64, t10069: f64, t18746: f64, t2718: f64, t6041: f64, t231: f64, t2782: f64, t2783: f64, t62868: f64, t14546: f64, t14547: f64, t18677: f64, t18681: f64, t2646: f64, t2724: f64, t2754: f64, t2811: f64, t4494: f64, t4514: f64, t51436: f64, t51564: f64, t51572: f64, t51576: f64, t51578: f64, t51587: f64, t61866: f64, t820: f64) -> f64 {
    let t62920 = t10073 * t18742;
    let t62922 = t10069 * t18746;
    let t62929 = t2718 * t6041;
    let t62938 = t2782 * t2783 * t62868 * t231;
    let t62945 = -0.79025390195226139182e1_f64 * t14546 * t18681 * t14547 + 0.46263278077393568556e-2_f64 * t51564 - 0.65854491829355115987e0_f64 * t4514 * t18677 * t2754 + 0.65049603595885220126e-3_f64 * t62920 - 0.14634331517634470219e-1_f64 * t62922 + 0.26341796731742046394e1_f64 * t820 * t2811 * t61866 + 0.65854491829355115984e-1_f64 * t51572 - 0.65854491829355115984e-1_f64 * t51576 + 0.13170898365871023197e1_f64 * t820 * t62929 * t2724 - 0.26341796731742046394e1_f64 * t4514 * t4494 * t51436 + 0.21951497276451705328e-1_f64 * t62938 - 0.22089088168956307394e-3_f64 * t51578 - 0.65854491829355115987e0_f64 * t4514 * t18677 * t2646 - 0.52039682876708176102e-1_f64 * t51587;
    t62945
}
