//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta490 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1485;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1486;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta490<F: Float>(t1412: F, t6861: F, t22212: F, t2496: F, t2626: F, t1320: F, t22195: F, t22129: F, t2713: F, t3964: F, t6856: F, t9779: F, t6880: F, t22062: F, t9775: F, t22068: F, t9765: F, t22022: F, t22061: F, t808: F, t9845: F, t22182: F, t47215: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t74026, t74106, t74130, t74132, t74264, t74277) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1485::<F>(t1412, t6861, t22212, t2496, t2626, t1320, t22195, t22129, t2713, t3964, t6856, t9779);
        let (t74279, t74281, t74290, t74299, t74304, t74322) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1486::<F>(t6880, t9779, t22062, t9775, t22068, t9765, t22022, t22061, t808, t9845, t22182, t47215);
    (t74026, t74106, t74130, t74132, t74264, t74277, t74279, t74281, t74290, t74299, t74304, t74322)
}
