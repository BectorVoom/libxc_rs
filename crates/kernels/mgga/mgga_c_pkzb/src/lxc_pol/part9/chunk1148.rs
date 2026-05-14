//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1148/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1148<F: Float>(t1134: F, t1144: F, t2112: F, t2119: F, t2145: F, t2146: F, t22060: F, t22119: F, t2957: F, t2964: F, t2989: F, t2990: F, t307: F, t5990: F, t6000: F, t6002: F, t6054: F, t7805: F, t7821: F, t7824: F, t786: F, t7885: F, t790: F, t800: F) -> (F,) {
    let t22124 = -0.11853808529283920877e2 * t307 * t6000 * t2989 * t2119 - 0.19756347548806534796e1 * t786 * t7885 - 0.19756347548806534796e1 * t2957 * t2146 - 0.19756347548806534796e1 * t7805 * t800 + 0.13170898365871023197e1 * t307 * t2964 * t6054 - 0.39512695097613069591e1 * t1134 * t6002 - 0.11853808529283920877e2 * t786 * t7821 - 0.19756347548806534796e1 * t2112 * t2990 + 0.39512695097613069591e1 * t307 * t7824 * t2145 - 0.65854491829355115987e0 * t5990 * t1144 - 0.65854491829355115987e0 * t307 * t790 * (t22060 + t22119);
    (t22124,)
}
