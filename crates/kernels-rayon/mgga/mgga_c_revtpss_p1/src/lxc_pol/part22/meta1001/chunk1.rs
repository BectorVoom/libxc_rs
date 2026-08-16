//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3407/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3407(t19133: f64, t2989: f64, t981: f64, t15559: f64, t4719: f64, t11591: f64, t6223: f64, t19049: f64, t3026: f64, t15556: f64, t19146: f64, t3007: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t63923 = 0.14035736694323150897e2_f64 * t981 * t19133 * t2989;
    let t63925 = 0.70178683471615754484e1_f64 * t4719 * t15559;
    let t63927 = 0.5848223622634646207e0_f64 * t11591 * t6223;
    let t63929 = 0.11696447245269292414e1_f64 * t19049 * t3026;
    let t63934 = 0.34631718211362927517e2_f64 * t4719 * t15556;
    let t63937 = 0.11696447245269292414e1_f64 * t981 * t19146 * t3007;
    (t63923, t63925, t63927, t63929, t63934, t63937)
}
