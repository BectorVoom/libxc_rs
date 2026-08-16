//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2001/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2001(t26016: f64, t92047: f64, t2031: f64, t96425: f64, t23967: f64, t27972: f64, t27976: f64, t2032: f64, t23963: f64, t23970: f64, t26009: f64, t26954: f64, t83717: f64, t90098: f64, t90114: f64, t91954: f64, t92057: f64, t96422: f64, t96443: f64, t96473: f64, t96535: f64) -> f64 {
    let t102173 = t26016 * t92047;
    let t102187 = t2031 * t96425;
    let t102192 = t23967 * t27972;
    let t102194 = t23967 * t27976;
    let t102198 = -160.0_f64 / 9.0_f64 * t102173 + 10.0_f64 / 3.0_f64 * t96473 * t23970 + 20.0_f64 / 3.0_f64 * t26016 * t92057 + 20.0_f64 * t91954 * t26009 + 20.0_f64 / 3.0_f64 * t90114 * t26954 + 20.0_f64 / 3.0_f64 * t96443 * t23970 + 20.0_f64 * t23963 * t96422 - 20.0_f64 * t83717 * t102187 + 20.0_f64 / 3.0_f64 * t90098 * t26954 + 80.0_f64 / 9.0_f64 * t102192 + 40.0_f64 / 9.0_f64 * t102194 - 2.0_f64 / 3.0_f64 * t96535 * t2032;
    t102198
}
