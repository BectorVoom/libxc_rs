//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1180/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1180(t26854: f64, t7687: f64, t93157: f64, t46978: f64, t7692: f64, t7690: f64, t1250: f64, t32896: f64, t2173: f64, t7710: f64, t10463: f64, t3489: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t93606 = t7687 * t26854;
    let t93628 = 0.73697530864197530862e-3_f64 * t93157;
    let t93661 = t46978 * t7692;
    let t93662 = t7690 * t93661;
    let t93737 = t32896 * t1250;
    let t93759 = t2173 * t46978 * t7710;
    let t93762 = t2173 * t93661;
    let t93779 = t10463 * t3489;
    (t93606, t93628, t93662, t93737, t93759, t93762, t93779)
}
