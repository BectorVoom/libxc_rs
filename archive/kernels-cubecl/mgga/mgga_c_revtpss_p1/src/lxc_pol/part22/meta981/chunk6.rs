//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3317/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3317<F: Float>(t10073: F, t18742: F, t10069: F, t18746: F, t2718: F, t6041: F, t231: F, t2782: F, t2783: F, t62868: F, t14546: F, t14547: F, t18677: F, t18681: F, t2646: F, t2724: F, t2754: F, t2811: F, t4494: F, t4514: F, t51436: F, t51564: F, t51572: F, t51576: F, t51578: F, t51587: F, t61866: F, t820: F) -> F {
    let t62920 = t10073 * t18742;
    let t62922 = t10069 * t18746;
    let t62929 = t2718 * t6041;
    let t62938 = t2782 * t2783 * t62868 * t231;
    let t62945 = -F::cast_from(0.79025390195226139182e1_f64) * t14546 * t18681 * t14547 + F::cast_from(0.46263278077393568556e-2_f64) * t51564 - F::cast_from(0.65854491829355115987e0_f64) * t4514 * t18677 * t2754 + F::cast_from(0.65049603595885220126e-3_f64) * t62920 - F::cast_from(0.14634331517634470219e-1_f64) * t62922 + F::cast_from(0.26341796731742046394e1_f64) * t820 * t2811 * t61866 + F::cast_from(0.65854491829355115984e-1_f64) * t51572 - F::cast_from(0.65854491829355115984e-1_f64) * t51576 + F::cast_from(0.13170898365871023197e1_f64) * t820 * t62929 * t2724 - F::cast_from(0.26341796731742046394e1_f64) * t4514 * t4494 * t51436 + F::cast_from(0.21951497276451705328e-1_f64) * t62938 - F::cast_from(0.22089088168956307394e-3_f64) * t51578 - F::cast_from(0.65854491829355115987e0_f64) * t4514 * t18677 * t2646 - F::cast_from(0.52039682876708176102e-1_f64) * t51587;
    t62945
}
