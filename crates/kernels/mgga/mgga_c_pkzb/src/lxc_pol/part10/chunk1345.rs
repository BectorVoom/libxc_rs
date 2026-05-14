//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1345/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1345<F: Float>(t1306: F, t2149: F, t2153: F, t25883: F, t25885: F, t25887: F, t25889: F, t25891: F, t25895: F, t25897: F, t25899: F, t25902: F, t25905: F, t25907: F, t26809: F, t26813: F, t7543: F, t9721: F) -> (F,) {
    let t26817 = 2.0 * t1306 * t2149 * t9721 + 2.0 * t1306 * t2153 * t26809 + 8.0 * t1306 * t26813 * t7543 - t25883 - t25885 + t25887 + t25889 - t25891 - t25895 - t25897 + t25899 + t25902 + t25905 - t25907;
    (t26817,)
}
