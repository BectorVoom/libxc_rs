//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta506 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1998;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1999;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta506<F: Float>(t1250: F, t20900: F, t482: F, t1042: F, t19680: F, t5268: F, t1247: F, t1261: F, t12910: F, t12956: F, t17339: F, t17396: F, t17505: F, t20858: F, t20864: F, t20868: F, t20876: F, t20880: F, t3708: F, t3711: F, t5299: F, t5354: F, t6619: F, t6625: F, t20823: F, t5265: F, t5274: F, t1774: F, t3362: F, t4181: F, t12787: F, t12916: F, t6689: F, t3718: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t20902, t20903, t20906, t20907, t20910) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1998::<F>(t1250, t20900, t482, t1042, t19680, t5268, t1247, t1261, t12910, t12956, t17339, t17396, t17505, t20858, t20864, t20868, t20876, t20880, t3708, t3711, t5299, t5354, t6619, t6625);
        let (t20913, t20914, t20917, t20922, t20923, t20926, t20927) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1999::<F>(t20823, t5268, t1042, t5265, t5274, t1774, t3362, t4181, t12787, t12916, t6689, t3718);
    (t20902, t20903, t20906, t20907, t20910, t20913, t20914, t20917, t20922, t20923, t20926, t20927)
}
