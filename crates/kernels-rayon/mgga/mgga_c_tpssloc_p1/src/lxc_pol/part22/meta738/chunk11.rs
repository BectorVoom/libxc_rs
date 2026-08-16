//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2433/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2433(t10828: f64, t14263: f64, t14337: f64, t17454: f64, t17493: f64, t17496: f64, t17500: f64, t21239: f64, t2905: f64, t2930: f64, t4454: f64, t4471: f64, t4476: f64, t49104: f64, t5794: f64, t60343: f64, t60424: f64, t69253: f64, t69255: f64, t69257: f64, t69259: f64, t69261: f64, t69263: f64, t69276: f64, t950: f64) -> f64 {
    let t69286 = t69253 - t69255 + t69257 - t69259 - t69261 - t69263 + 0.51947577317044391276e2_f64 * t14337 * t17493 + 0.10389515463408878255e3_f64 * t14337 * t17496 + 0.30762056574649219972e4_f64 * t49104 * t17500 - 0.31168546390226634765e3_f64 * t10828 * t5794 * t4471 - 0.11696447245269292414e1_f64 * t2905 * t21239 * t950 + 0.17315859105681463759e2_f64 * t2930 * t69276 * t950 - 0.35089341735807877242e1_f64 * t60424 * t4454 + 0.51947577317044391276e2_f64 * t60343 * t4476 - 0.35089341735807877242e1_f64 * t14263 * t17454;
    t69286
}
