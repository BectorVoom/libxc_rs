//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1274/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1274<F: Float>(t16721: F, t16775: F, t16779: F, t16783: F, t16787: F, t16906: F, t16909: F, t16915: F, t24648: F, t24649: F, t24650: F, t135: F, t1535: F, t1633: F, t1692: F, t16923: F, t19744: F, t24091: F, t24652: F, t24654: F, t24655: F, t24656: F, t24657: F, t24658: F, t2537: F, t2575: F, t2718: F, t6806: F, t7201: F, t8779: F, t9116: F) -> (F, F) {
    let t25029 = t16721 - t16775 - t16779 + t16783 - t16787 - t24648 - t16906 + t16909 - t24649 + t16915 + t24650;
    let t25045 = 12.0 * t135 * t1633 * t24091 - 6.0 * t1535 * t19744 * t2537 + 12.0 * t1535 * t2575 * t7201 + 12.0 * t1535 * t6806 * t8779 + 6.0 * t1692 * t2718 * t9116 - t16923 - t24652 - t24654 - t24655 - t24656 - t24657 - t24658;
    (t25029, t25045)
}
