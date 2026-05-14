//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1128/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1128<F: Float>(t6072: F, t779: F, t689: F, t1580: F, t4321: F, t6042: F, t786: F, t789: F, t6049: F, t14987: F, t4481: F, t11040: F, t15011: F, t15062: F, t15063: F, t2765: F, t4474: F, t4487: F, t4534: F) -> (F,) {
    let t18811 = t779 * t6072;
    let t18812 = t689 * t18811;
    let t18814 = t4321 * t1580;
    let t18815 = t689 * t18814;
    let t18821 = t786 * t6042;
    let t18822 = t18821 * t789;
    let t18825 = t779 * t6049;
    let t18826 = t689 * t18825;
    let t18828 = t14987 * t4481;
    let t18836 = 0.54878743191129263322e-2 * t18812 + 0.10975748638225852664e-1 * t18815 + 0.13170898365871023197e1 * t2765 * t6049 + 0.26341796731742046394e1 * t4474 * t4487 + t15062 + 0.9757440539382783019e-2 * t18822 + 0.14634331517634470219e-1 * t15063 - t11040 - 0.10975748638225852664e-1 * t18826 - 0.19514881078765566037e-1 * t18828 - 0.13170898365871023197e1 * t15011 * t1580 - 0.13170898365871023197e1 * t4474 * t4534 - 0.65854491829355115987e0 * t2765 * t6072;
    (t18836,)
}
