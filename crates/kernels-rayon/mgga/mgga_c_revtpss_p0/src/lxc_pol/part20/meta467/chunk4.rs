//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1790/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1790(t9590: f64, t9593: f64, t1353: f64, t13625: f64, t25802: f64, t3829: f64, t3889: f64, t39773: f64, t39783: f64, t39786: f64, t39791: f64, t39795: f64, t4139: f64, t47006: f64, t47008: f64, t47010: f64, t47012: f64, t47014: f64, t47017: f64, t5536: f64, t9599: f64) -> f64 {
    let t47638 = t9590 * t9593;
    let t47648 = 24.0_f64 * t1353 * t4139 * t47638 - 36.0_f64 * t13625 * t25802 * t4139 - 36.0_f64 * t3829 * t5536 * t9599 - 18.0_f64 * t3889 * t4139 * t9599 + t39773 - t39783 - t39786 - t39791 - t39795 + t47006 - t47008 + t47010 - t47012 + t47014 + t47017;
    t47648
}
