//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 573/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk573(t1008: f64, t1581: f64, t4797: f64, t4799: f64, t4808: f64, t4816: f64, t3228: f64, t532: f64, t1569: f64, t3670: f64, t542: f64, t537: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4846 = 0.85748036236139473944e-3_f64 * t1008 * t1581;
    let t4856 = t4797 / 6.0_f64;
    let t4857 = 2.0_f64 / 3.0_f64 * t4799;
    let t4860 = t4808 / 12.0_f64;
    let t4863 = 4.0_f64 / 3.0_f64 * t4816;
    let t4881 = t3228 * t532;
    let t4884 = 0.17149607247227894789e-2_f64 * t1008 * t1569;
    let t4889 = t3670 * t532;
    let t4891 = t3670 * t542;
    let t4897 = t3670 * t537;
    (t4846, t4856, t4857, t4860, t4863, t4881, t4884, t4889, t4891, t4897)
}
