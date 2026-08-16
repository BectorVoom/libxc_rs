//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 956/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk956(t3255: f64, t7214: f64, t518: f64, t7141: f64, t1319: f64, t3786: f64, t1419: f64, t7142: f64, t5498: f64, t1889: f64, t3766: f64, t5526: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22091 = t3255 * t7214;
    let t22093 = t518 * t7141;
    let t22094 = t22093 * t1319;
    let t22095 = t3786 * t22094;
    let t22098 = t7142 * t1419;
    let t22099 = t5498 * t22098;
    let t22103 = t3766 * t1889 * t5526;
    (t22091, t22094, t22095, t22098, t22099, t22103)
}
