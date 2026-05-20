//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3832/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3832<F: Float>(t22452: F, t2470: F, t9680: F, t1444: F, t2782: F, t556: F, t6895: F, t9656: F, t22409: F, t2435: F, t13730: F, t1893: F) -> (F, F, F, F) {
    let t73666 = t9680 * t22452 * t2470;
    let t73671 = t2782 * t556 * t9656 * t6895 * t1444;
    let t73673 = t2435 * t22409;
    let t73676 = t2782 * t1893 * t13730;
    (t73666, t73671, t73673, t73676)
}
