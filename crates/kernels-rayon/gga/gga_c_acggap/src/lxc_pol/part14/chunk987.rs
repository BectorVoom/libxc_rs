//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 987/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk987(t34893: f64, t7440: f64, t8929: f64, t2282: f64, t7600: f64, t174: f64, t7815: f64, t1181: f64, t20992: f64, t7351: f64, t7426: f64, t1983: f64, t30127: f64, t7586: f64, t8791: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34894 = 0.16809375e0_f64 * t34893;
    let t34895 = t7440 * t8929;
    let t34896 = 0.16809375e0_f64 * t34895;
    let t34897 = t7600 * t2282;
    let t34903 = t7815 * t174;
    let t34945 = t7426 * t1181 * t7351 * t20992;
    let t34946 = 0.18868855373762491241e-2_f64 * t34945;
    let t34957 = t30127 * t7586 * t1983 * t8791;
    (t34894, t34896, t34897, t34903, t34946, t34957)
}
