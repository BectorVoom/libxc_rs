//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 744/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk744(t6063: f64, t6064: f64, t2155: f64, t110: f64, t1603: f64, t2161: f64, t2: f64, t386: f64, t481: f64, t506: f64, t2106: f64, t776: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6065 = t6063 * t6064;
    let t6066 = t2155 * t6065;
    let t6068 = t1603 * t110;
    let t6069 = t2161 * t6068;
    let t6072 = t506 * t2 * t386 * t481;
    let t6073 = t6069 * t6072;
    let t6075 = t776 * t2106;
    (t6066, t6068, t6069, t6072, t6073, t6075)
}
