//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2503/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2503<F: Float>(t2439: F, t2440: F, t4534: F, t1580: F, t41117: F, t10509: F, t10995: F, t14990: F, t10868: F, t820: F, t844: F, t14701: F, t40731: F) -> (F, F, F, F, F) {
    let t50245 = t2439 * t2440 * t4534;
    let t50248 = t41117 * t1580;
    let t50253 = t10995 * t14990 * t10509;
    let t50295 = t820 * t10868 * t844;
    let t50298 = t40731 * t14701;
    (t50245, t50248, t50253, t50295, t50298)
}
