//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 999/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk999(t12285: f64, t339: f64, t341: f64, t1127: f64, t2410: f64, t1020: f64, t3522: f64, t3745: f64, t839: f64, t333: f64, t335: f64, t337: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12286 = t339 * t12285;
    let t12288 = t341 * t12285;
    let t12292 = t2410 * t1127;
    let t12294 = t1020 * t3522;
    let t12296 = t839 * t3745;
    let t12298 = t333 * t12285;
    let t12300 = t335 * t12285;
    let t12302 = t337 * t12285;
    (t12286, t12288, t12292, t12294, t12296, t12298, t12300, t12302)
}
