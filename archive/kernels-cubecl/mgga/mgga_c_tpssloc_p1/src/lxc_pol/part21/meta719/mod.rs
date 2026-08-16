//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta719 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2562;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2563;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta719<F: Float>(t2770: F, t2987: F, t10277: F, t4509: F, t10390: F, t13765: F, t10937: F, t14501: F, t1606: F, t2402: F, t973: F, t10454: F, t4644: F, t13950: F, t3117: F, t14202: F, t3048: F, t14206: F, t3108: F, t3185: F, t49649: F, t10470: F, t11058: F, t381: F, t1615: F, t6739: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t50366, t50370, t50378, t50384, t50425, t50429) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2562::<F>(t2770, t2987, t10277, t4509, t10390, t13765, t10937, t14501, t1606, t2402, t973, t10454, t4644);
        let (t50438, t50442, t50445, t50465, t50508, t50509) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2563::<F>(t13950, t3117, t14202, t3048, t14206, t3108, t3185, t49649, t10470, t11058, t381, t1615, t6739);
    (t50366, t50370, t50378, t50384, t50425, t50429, t50438, t50442, t50445, t50465, t50508, t50509)
}
