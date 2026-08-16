//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1499/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1499<F: Float>(t18742: F, t2782: F, t18681: F, t231: F, t2783: F, t18677: F, t2723: F, t4503: F, t10916: F, t14577: F, t14581: F, t14590: F, t14596: F, t14603: F, t14608: F, t14948: F, t18727: F, t18731: F, t18733: F, t18739: F) -> F {
    let t18743 = t2782 * t18742;
    let t18746 = t2783 * t18681 * t231;
    let t18747 = t2782 * t18746;
    let t18750 = t4503 * t18677 * t2723;
    let t18751 = t2782 * t18750;
    let t18754 = -t14577 + F::cast_from(0.14634331517634470219e-1_f64) * t14581 - F::cast_from(0.9757440539382783019e-2_f64) * t18727 - F::cast_from(0.9757440539382783019e-2_f64) * t18731 - t14590 - F::cast_from(0.19514881078765566037e-1_f64) * t18733 + F::cast_from(0.11565819519348392139e-2_f64) * t10916 + t14596 + F::cast_from(0.39029762157531132076e-1_f64) * t14603 + F::cast_from(0.54878743191129263322e-2_f64) * t18739 + F::cast_from(0.54878743191129263322e-2_f64) * t18743 + F::cast_from(0.10975748638225852664e-1_f64) * t18747 - F::cast_from(0.10975748638225852664e-1_f64) * t18751 - t14608 + F::cast_from(0.23131639038696784278e-2_f64) * t14948;
    t18754
}
