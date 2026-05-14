//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 518/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk518<F: Float>(t1469: F, t2852: F, t2857: F, t1596: F, t914: F, t1600: F, t2880: F, t2897: F, t1606: F, t698: F, t1614: F, t945: F, t1626: F, t964: F, t1633: F, t3014: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4573 = t2852 * t1469;
    let t4578 = t2857 * t1469;
    let t4590 = t1596 * t914;
    let t4598 = t2880 * t1600;
    let t4614 = t2897 * t1600;
    let t4620 = t698 * t1606;
    let t4647 = t1614 * t945;
    let t4685 = t1626 * t964;
    let t4711 = t1633 * t3014;
    (t4573, t4578, t4590, t4598, t4614, t4620, t4647, t4685, t4711)
}
