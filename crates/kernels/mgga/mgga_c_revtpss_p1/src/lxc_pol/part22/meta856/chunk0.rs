//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3001/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3001<F: Float>(t2439: F, t4469: F, t780: F, t785: F, t213: F, t252: F, t2440: F, t4534: F, t1580: F, t41117: F, t10509: F, t10995: F, t14990: F) -> (F, F, F, F, F) {
    let t50236 = t2439 * t785 * t4469 * t780;
    let t50240 = t213 * t252;
    let t50245 = t2439 * t2440 * t4534;
    let t50248 = t41117 * t1580;
    let t50253 = t10995 * t14990 * t10509;
    (t50236, t50240, t50245, t50248, t50253)
}
