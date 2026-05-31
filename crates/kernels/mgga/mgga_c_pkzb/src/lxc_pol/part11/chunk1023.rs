//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1023/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1023<F: Float>(t1209: F, t3819: F, t1208: F, t9985: F, t11155: F, t6348: F, t7955: F, t9782: F, t11236: F, t11238: F, t11240: F, t11242: F, t11245: F, t11263: F, t11266: F, t11269: F, t11287: F, t11292: F, t11295: F, t11296: F, t11299: F, t2257: F, t2279: F, t2296: F, t2318: F, t365: F, t3796: F, t6313: F, t8120: F, t863: F) -> (F, F, F, F) {
    let t11302 = t1209 * t3819;
    let t11305 = t9985 * t1208;
    let t11311 = -t6348 + F::cast_from(0.68493333333333333332e-1_f64) * t7955 - F::cast_from(0.51369999999999999999e-1_f64) * t9782 + F::cast_from(0.5137e-1_f64) * t11155;
    let t11314 = -t11236 - t11238 - t11240 - t11242 + t11245 - t11263 - t11266 + F::cast_from(0.96491876992155210402e2_f64) * t8120 * t3796 - F::cast_from(0.19298375398431042081e3_f64) * t6313 * t11269 + F::cast_from(1.0_f64) * t863 * t11287 + t11292 - t11295 - F::cast_from(6.0_f64) * t2257 * t11296 + F::cast_from(0.96491876992155210402e2_f64) * t2279 * t11299 - F::cast_from(0.35089341735807877242e1_f64) * t2296 * t11302 + F::cast_from(0.51947577317044391277e2_f64) * t2318 * t11305 - F::cast_from(0.310907e-1_f64) * t11311 * t365;
    (t11302, t11305, t11311, t11314)
}
