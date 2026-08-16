//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1146/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1146(t2333: f64, t3060: f64, t795: f64, t3229: f64, t792: f64, t8601: f64, t12414: f64, t2892: f64, t12574: f64, t23495: f64, t3629: f64, t11888: f64, t8358: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42418 = t2333 * t3060;
    let t42419 = t42418 * t795;
    let t42423 = t2333 * t3229;
    let t42424 = t42423 * t795;
    let t42428 = t8601 * t792;
    let t42432 = t12414 * t792;
    let t42453 = t2333 * t2892;
    let t42454 = t42453 * t795;
    let t42472 = t12574 * t792;
    let t42491 = t23495 * t3629;
    let t42493 = t8358 * t11888;
    (t42419, t42424, t42428, t42432, t42454, t42472, t42491, t42493)
}
