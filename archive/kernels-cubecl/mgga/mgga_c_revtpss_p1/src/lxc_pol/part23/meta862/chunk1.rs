//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2753/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2753<F: Float>(t21931: F, t72: F, t757: F, t6922: F, t9593: F, t22185: F, t2619: F, t22404: F, t3920: F, t1445: F, t22445: F, t689: F) -> (F, F, F, F, F) {
    let t73493 = t21931 * t72 * t757;
    let t73499 = t6922 * t9593;
    let t73515 = t22185 * t2619;
    let t73587 = t22404 * t3920;
    let t73590 = t689 * t22445 * t1445;
    (t73493, t73499, t73515, t73587, t73590)
}
