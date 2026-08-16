//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 833/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk833(t10416: f64, t18613: f64, t934: f64, t330: f64, t3303: f64, t1045: f64, t829: f64, t14128: f64, t14326: f64, t3288: f64, t18534: f64, t14301: f64, t18535: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t18615 = t10416 * t18613 * t934;
    let t18618 = t3303 * t330;
    let t18619 = t18613 * t1045;
    let t18620 = t18618 * t18619;
    let t18623 = t18613 * t829;
    let t18624 = t14128 * t18623;
    let t18627 = t14326 * t18623;
    let t18630 = t3288 * t330;
    let t18632 = t18630 * t18534 * t934;
    let t18636 = t10416 * t18534 * t1045;
    let t18639 = t14301 * t18535;
    (t18615, t18619, t18620, t18623, t18624, t18627, t18632, t18636, t18639)
}
