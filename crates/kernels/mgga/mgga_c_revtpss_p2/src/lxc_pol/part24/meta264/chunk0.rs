//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1035/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1035<F: Float>(t17376: F, t3599: F, t1285: F, t17395: F, t1781: F, t697: F, t1222: F, t3367: F, t471: F, t372: F, t5296: F, t17350: F, t3767: F) -> (F, F, F, F, F, F, F) {
    let t17572 = t17376 * t3599;
    let t17605 = t1285 * t17395;
    let t17628 = t697 * t1781;
    let t17629 = t1222 * t17628;
    let t17643 = t471 * t3367;
    let t17649 = t372 * t5296;
    let t17654 = t3767 * t17350;
    (t17572, t17605, t17628, t17629, t17643, t17649, t17654)
}
