//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2756/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2756<F: Float>(t1444: F, t2782: F, t556: F, t6895: F, t9656: F, t22409: F, t2435: F, t13730: F, t1893: F, t3899: F, t689: F, t6919: F) -> (F, F, F, F) {
    let t73671 = t2782 * t556 * t9656 * t6895 * t1444;
    let t73673 = t2435 * t22409;
    let t73676 = t2782 * t1893 * t13730;
    let t73705 = t689 * t3899 * t6919;
    (t73671, t73673, t73676, t73705)
}
