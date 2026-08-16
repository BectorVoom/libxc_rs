//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1287/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1287(t11227: f64, t11302: f64, t11305: f64, t1208: f64, t1209: f64, t2296: f64, t2318: f64, t27795: f64, t31327: f64, t31329: f64, t31331: f64, t31333: f64, t31335: f64, t31337: f64, t31339: f64, t3135: f64, t3136: f64, t3807: f64, t3819: f64, t6266: f64, t6300: f64, t6323: f64, t8071: f64, t889: f64, t9878: f64, t9929: f64) -> f64 {
    let t31345 = -0.70178683471615754484e1_f64 * t8071 * t9878 - 0.14035736694323150897e2_f64 * t6323 * t11227 * t889 + 0.10526802520742363173e2_f64 * t2318 * t3807 * t3135 - 0.35089341735807877242e1_f64 * t6266 * t11302 - 0.35089341735807877242e1_f64 * t2296 * t3136 * t3819 - 0.35089341735807877242e1_f64 * t2296 * t1209 * t9929 + t31327 - t31329 + t31331 - t31333 + t31335 - t31337 - t31339 + 0.51947577317044391277e2_f64 * t6300 * t11305 + 0.51947577317044391277e2_f64 * t2318 * t27795 * t1208;
    t31345
}
