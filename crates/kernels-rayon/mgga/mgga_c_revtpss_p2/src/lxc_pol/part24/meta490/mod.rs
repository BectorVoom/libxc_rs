//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta490 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1485;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1486;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta490(t1412: f64, t6861: f64, t22212: f64, t2496: f64, t2626: f64, t1320: f64, t22195: f64, t22129: f64, t2713: f64, t3964: f64, t6856: f64, t9779: f64, t6880: f64, t22062: f64, t9775: f64, t22068: f64, t9765: f64, t22022: f64, t22061: f64, t808: f64, t9845: f64, t22182: f64, t47215: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t74026, t74106, t74130, t74132, t74264, t74277) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1485(t1412, t6861, t22212, t2496, t2626, t1320, t22195, t22129, t2713, t3964, t6856, t9779);
        let (t74279, t74281, t74290, t74299, t74304, t74322) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1486(t6880, t9779, t22062, t9775, t22068, t9765, t22022, t22061, t808, t9845, t22182, t47215);
    (t74026, t74106, t74130, t74132, t74264, t74277, t74279, t74281, t74290, t74299, t74304, t74322)
}
