//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta373 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1264;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1265;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta373<F: Float>(t12839: F, t1469: F, t20795: F, t3626: F, t6638: F, t17304: F, t17340: F, t17342: F, t17438: F, t1791: F, t20817: F, t20843: F, t20847: F, t20851: F, t20917: F, t20927: F, t20966: F, t21177: F, t5331: F, t5340: F, t6611: F, t1715: F, t21093: F, t1042: F, t1774: F, t5819: F, t5268: F, t6573: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t24567, t24568, t24569, t24572, t24573, t24587) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1264::<F>(t12839, t1469, t20795, t3626, t6638, t17304, t17340, t17342, t17438, t1791, t20817, t20843, t20847, t20851, t20917, t20927, t20966, t21177, t5331, t5340, t6611);
        let (t24604, t24605, t24610, t24611, t24612, t24616) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1265::<F>(t1715, t21093, t1042, t1774, t5819, t5268, t6573);
    (t24567, t24568, t24569, t24572, t24573, t24587, t24604, t24605, t24610, t24611, t24612, t24616)
}
