//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 146/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk146(t397: f64, t399: f64, t539: f64, t535: f64, t473: f64, t524: f64, t489: f64, t501: f64, t240: f64, t505: f64, rho1: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t541 = t397 * t399 * t539;
    let t544 = 1.0_f64 + 0.2698618307426597582e-1_f64 * t535 * t541;
    let t545 = f64::ln(t544);
    let t547 = 1.0_f64 + 0.193e0_f64 * t545;
    let t548 = 1.0_f64 / t547;
    let t551 = t524 * t548 + 0.17411041666666666666e-2_f64 * t473;
    let t554 = 1.0_f64 + 0.9375e-1_f64 * t489 - 0.101171875e-1_f64 * t501;
    let t555 = 1.0_f64 / t554;
    let t559 = t505 + t240 * (t551 * t555 - t505);
    let t563 = 1.0_f64 / rho1;
    let t564 = sigma2 * t563;
    (t541, t544, t547, t548, t551, t554, t555, t559, t564)
}
