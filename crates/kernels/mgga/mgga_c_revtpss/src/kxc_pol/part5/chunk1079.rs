//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1079/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1079<F: Float>(t6017: F, t72: F, t686: F, t2798: F, t5978: F, t14568: F, t4500: F, t18699: F, t231: F, t2783: F, t2782: F, t18677: F, t18681: F, t2723: F, t4503: F, t10916: F, t14577: F, t14581: F, t14590: F, t14596: F, t14603: F, t14608: F, t14948: F) -> (F,) {
    let t18725 = t6017 * t72;
    let t18726 = t18725 * t686;
    let t18727 = t2798 * t18726;
    let t18729 = t5978 * t72;
    let t18730 = t18729 * t686;
    let t18731 = t2798 * t18730;
    let t18733 = t14568 * t4500;
    let t18738 = t2783 * t18699 * t231;
    let t18739 = t2782 * t18738;
    let t18742 = t2783 * t18677 * t231;
    let t18743 = t2782 * t18742;
    let t18746 = t2783 * t18681 * t231;
    let t18747 = t2782 * t18746;
    let t18750 = t4503 * t18677 * t2723;
    let t18751 = t2782 * t18750;
    let t18754 = -t14577 + 0.14634331517634470219e-1 * t14581 - 0.9757440539382783019e-2 * t18727 - 0.9757440539382783019e-2 * t18731 - t14590 - 0.19514881078765566037e-1 * t18733 + 0.11565819519348392139e-2 * t10916 + t14596 + 0.39029762157531132076e-1 * t14603 + 0.54878743191129263322e-2 * t18739 + 0.54878743191129263322e-2 * t18743 + 0.10975748638225852664e-1 * t18747 - 0.10975748638225852664e-1 * t18751 - t14608 + 0.23131639038696784278e-2 * t14948;
    (t18754,)
}
