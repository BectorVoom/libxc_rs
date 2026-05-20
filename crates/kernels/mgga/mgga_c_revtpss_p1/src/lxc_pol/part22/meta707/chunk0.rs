//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2730/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2730<F: Float>(t1868: F, t4003: F, t6843: F, t2723: F, t6016: F, t1544: F, t11660: F, t1469: F, t159: F, t2698: F, t1518: F, t648: F) -> (F, F, F, F, F, F, F) {
    let t22841 = t4003 * t1868;
    let t23037 = t4003 * t6843;
    let t23160 = t2723 * t6016;
    let t23334 = t2723 * t1544;
    let t23898 = t11660 * t1469;
    let t25273 = t2698 * t159;
    let t27123 = t648 * t1518;
    (t22841, t23037, t23160, t23334, t23898, t25273, t27123)
}
