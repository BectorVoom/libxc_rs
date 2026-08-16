//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1512/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1512(t1007: f64, t11738: f64, t3080: f64, t3083: f64, t1043: f64, t11173: f64, t11858: f64, t16048: f64, t11859: f64, t11861: f64, t11922: f64, t11927: f64, t11929: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42754 = t11738 * t1007;
    let t42756 = t3083 * t3080;
    let t42760 = t11173 * t1043;
    let t42765 = t11858 * t16048;
    let t42769 = t11859 * t11922 * t11861;
    let t42772 = t11927 * t11922 * t11929;
    (t42754, t42756, t42760, t42765, t42769, t42772)
}
