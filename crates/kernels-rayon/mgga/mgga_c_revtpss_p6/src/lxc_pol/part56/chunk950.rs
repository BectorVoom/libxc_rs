//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 950/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk950(t1936: f64, t29432: f64, t7002: f64, t7586: f64, t1937: f64, t27060: f64, t6993: f64, t7316: f64, t8764: f64, t7239: f64, t2163: f64, t651: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32830 = t29432 * t1936;
    let t32832 = t7586 * t7002;
    let t32840 = t27060 * t1937;
    let t32843 = t29432 * t1937;
    let t32845 = t7586 * t6993;
    let t32849 = t8764 * t7316;
    let t32850 = t8764 * t7239;
    let t32855 = t2163 * t7002;
    let t32856 = t651 * t32855;
    (t32830, t32832, t32840, t32843, t32845, t32849, t32850, t32855, t32856)
}
