//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1017/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1017(t1081: f64, t2410: f64, t1020: f64, t3386: f64, t3648: f64, t839: f64, t11930: f64, t333: f64, t335: f64, t337: f64, t339: f64, t341: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11975 = t2410 * t1081;
    let t11977 = t1020 * t3386;
    let t11979 = t839 * t3648;
    let t11981 = t333 * t11930;
    let t11983 = t335 * t11930;
    let t11985 = t337 * t11930;
    let t11987 = t339 * t11930;
    let t11989 = t341 * t11930;
    (t11975, t11977, t11979, t11981, t11983, t11985, t11987, t11989)
}
