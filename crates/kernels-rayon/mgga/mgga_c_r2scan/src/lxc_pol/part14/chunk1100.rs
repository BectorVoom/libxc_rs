//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1100/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1100(t3270: f64, t39014: f64, t1114: f64, t6897: f64, t2330: f64, t3492: f64, t5086: f64, t37358: f64, t37386: f64, t37397: f64, t37406: f64, t37412: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t39015 = t3270 * t39014;
    let t39030 = t1114 * t6897;
    let t39032 = t3270 * t39030 * t2330;
    let t39040 = t5086 * t3492;
    let t39046 = 0.26021382394247697185e-3_f64 * t37358;
    let t39054 = 0.205201155180140685e-5_f64 * t37386;
    let t39059 = 0.487802396665200453e-2_f64 * t37397;
    let t39061 = 0.11709622077411463733e-2_f64 * t37406;
    let t39062 = 0.18292589874945016987e-2_f64 * t37412;
    (t39015, t39030, t39032, t39040, t39046, t39054, t39059, t39061, t39062)
}
