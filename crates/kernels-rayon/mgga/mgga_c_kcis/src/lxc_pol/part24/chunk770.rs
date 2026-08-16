//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 770/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk770(t357: f64, t359: f64, t373: f64, t9587: f64, t1164: f64, t3225: f64, t334: f64, t369: f64, t86: f64, t1143: f64, t245: f64, t1157: f64, t752: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10506 = 1.0_f64 / t359 / t357;
    let t10513 = t373 * t9587;
    let t10525 = t1164 * t3225;
    let t10526 = t10525 * sigma0;
    let t10541 = 0.11791604938271604938e-1_f64 * t86 * t334 * t369;
    let t10544 = t1143 * t245;
    let t10556 = t752 * t1157;
    (t10506, t10513, t10525, t10526, t10541, t10544, t10556)
}
