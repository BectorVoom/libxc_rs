//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1023/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1023(t1209: f64, t3819: f64, t1208: f64, t9985: f64, t11155: f64, t6348: f64, t7955: f64, t9782: f64, t11236: f64, t11238: f64, t11240: f64, t11242: f64, t11245: f64, t11263: f64, t11266: f64, t11269: f64, t11287: f64, t11292: f64, t11295: f64, t11296: f64, t11299: f64, t2257: f64, t2279: f64, t2296: f64, t2318: f64, t365: f64, t3796: f64, t6313: f64, t8120: f64, t863: f64) -> (f64, f64, f64, f64) {
    let t11302 = t1209 * t3819;
    let t11305 = t9985 * t1208;
    let t11311 = -t6348 + 0.68493333333333333332e-1_f64 * t7955 - 0.51369999999999999999e-1_f64 * t9782 + 0.5137e-1_f64 * t11155;
    let t11314 = -t11236 - t11238 - t11240 - t11242 + t11245 - t11263 - t11266 + 0.96491876992155210402e2_f64 * t8120 * t3796 - 0.19298375398431042081e3_f64 * t6313 * t11269 + 1.0_f64 * t863 * t11287 + t11292 - t11295 - 6.0_f64 * t2257 * t11296 + 0.96491876992155210402e2_f64 * t2279 * t11299 - 0.35089341735807877242e1_f64 * t2296 * t11302 + 0.51947577317044391277e2_f64 * t2318 * t11305 - 0.310907e-1_f64 * t11311 * t365;
    (t11302, t11305, t11311, t11314)
}
