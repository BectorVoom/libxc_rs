//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2266/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2266<F: Float>(t30923: F, t3801: F, t105665: F, t105669: F, t111864: F, t111913: F, t111959: F, t112009: F, t112051: F, t112092: F, t112138: F, t112564: F, t112602: F, t112645: F, t112697: F, t112744: F, t112787: F, t112846: F, t112899: F, t112950: F, t1298: F, t1300: F, t1832: F, t198: F, t21635: F, t27037: F, t27041: F, t29317: F, t29322: F, t336: F, t5023: F, t5501: F, t6748: F, t6752: F, t7673: F, t97491: F, t97498: F) -> F {
    let t112958 = t30923 * t3801;
    let t112989 = t198 * t336 * (t111864 + t111913 + t111959 + t112009 + t112051 + t112092 + t112138 + t112564 + t112602 + t112645 + t112697 + t112744 + t112787 + t112846 + t112899 + t112950) * t1300 - t5023 * t112958 * t1298 - F::new(2.0) * t5023 * t105665 * t1832 + F::new(4.0) * t5023 * t105669 * t29322 - F::new(2.0) * t5023 * t29317 * t5501 + F::new(2.0) * t5023 * t97491 * t6752 - F::new(6.0) * t5023 * t97498 * t6752 * t1298 + F::new(4.0) * t5023 * t27041 * t1832 * t5501 - t5023 * t27037 * t6748 + F::new(2.0) * t5023 * t27041 * t6748 * t1298 - t5023 * t7673 * t21635;
    t112989
}
