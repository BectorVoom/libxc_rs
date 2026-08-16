//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 941/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk941(t13803: f64, t13849: f64, t13889: f64, t13932: f64, t1328: f64, t3579: f64, t3764: f64, t1340: f64, t1339: f64, t1440: f64, t3732: f64, t1341: f64) -> (f64, f64, f64, f64) {
    let t13934 = t13803 + t13849 + t13889 + t13932;
    let t13935 = t13934 * t1328;
    let t13938 = t3764 * t3579;
    let t13939 = t1340 * t13938;
    let t13940 = t1339 * t13939;
    let t13944 = t3732 * t1440;
    let t13945 = t1341 * t13944;
    (t13935, t13940, t13944, t13945)
}
