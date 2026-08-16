//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta451 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1415;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1416;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta451<F: Float>(t14362: F, t9863: F, t9866: F, t10115: F, t1570: F, t4322: F, t9292: F, t10981: F, t1579: F, t22: F, t868: F, t2465: F, t4480: F, t9288: F, t1569: F, t2769: F, t786: F, t10985: F, t15017: F, t1580: F, t41117: F, t1565: F, t40781: F, t40488: F, t4354: F, t268: F, t40452: F, t4371: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t50092, t50094, t50155, t50166, t50178, t50205) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1415::<F>(t14362, t9863, t9866, t10115, t1570, t4322, t9292, t10981, t1579, t22, t868, t2465, t4480, t9288);
        let (t50208, t50214, t50248, t50370, t50372, t50377) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1416::<F>(t1569, t2769, t786, t10985, t15017, t1580, t41117, t1565, t40781, t40488, t4354, t268, t40452, t4371);
    (t50092, t50094, t50155, t50166, t50178, t50205, t50208, t50214, t50248, t50370, t50372, t50377)
}
