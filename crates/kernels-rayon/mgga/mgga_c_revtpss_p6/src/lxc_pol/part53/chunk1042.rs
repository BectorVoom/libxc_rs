//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1042/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1042(t2163: f64, t7002: f64, t651: f64, t7003: f64, t7586: f64, t2322: f64, t8749: f64, t4254: f64, t1936: f64, t7683: f64, t670: f64, t8756: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32855 = t2163 * t7002;
    let t32856 = t651 * t32855;
    let t32858 = t7586 * t7003;
    let t32862 = t2322 * t8749;
    let t32864 = t4254 * t8749;
    let t32866 = t7683 * t1936;
    let t32867 = t651 * t32866;
    let t32869 = t8756 * t670;
    (t32855, t32856, t32858, t32862, t32864, t32866, t32867, t32869)
}
