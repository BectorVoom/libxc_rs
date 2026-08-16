//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 672/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk672(t1268: f64, t1270: f64, t1190: f64, t2222: f64, t1183: f64, t72: f64, t732: f64, t1193: f64, t2345: f64, t2215: f64, t724: f64, t489: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3184 = t1268 * t1270;
    let t3189 = 0.24415263074675393405e-3_f64 * t1190 * t2222;
    let t3190 = t1183 * t72;
    let t3191 = t3190 * t732;
    let t3192 = 0.36622894612013090108e-3_f64 * t3191;
    let t3194 = 0.11696447245269292414e1_f64 * t1193 * t2345;
    let t3196 = 0.17315859105681463759e2_f64 * t1193 * t2215;
    let t3197 = t1183 * t724;
    let t3198 = t489 * t3197;
    (t3184, t3189, t3190, t3191, t3192, t3194, t3196, t3197, t3198)
}
