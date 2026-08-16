//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1265/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1265(t19113: f64, t303: f64, t7726: f64, t2822: f64, t28904: f64, t27924: f64, t4773: f64, t6481: f64, t7731: f64, t1014: f64, t28944: f64, t27836: f64, t2842: f64, t4556: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t100678 = t303 * t7726 * t19113;
    let t100680 = t2822 * t28904;
    let t100683 = t303 * t27924 * t4773;
    let t100686 = t303 * t6481 * t7731;
    let t100688 = t1014 * t28944;
    let t100691 = t2842 * t27836 * t4556;
    (t100678, t100680, t100683, t100686, t100688, t100691)
}
