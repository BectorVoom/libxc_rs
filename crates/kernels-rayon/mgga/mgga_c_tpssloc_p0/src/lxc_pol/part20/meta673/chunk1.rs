//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2539/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2539(t3316: f64, t51402: f64, t11300: f64, t11361: f64, t11430: f64, t11437: f64, t11441: f64, t1155: f64, t15126: f64, t15219: f64, t15222: f64, t43984: f64, t44188: f64, t4862: f64, t51133: f64, t51245: f64, t51248: f64, t51251: f64, t51382: f64, t51385: f64, t51389: f64, t51392: f64, t51399: f64, t51401: f64) -> (f64, f64) {
    let t51404 = 0.48245938496077605201e2_f64 * t51402 * t3316;
    let t51411 = 18.0_f64 * t51382 * t11437 - t51133 - t51245 + 0.30762056574649219974e4_f64 * t51385 * t43984 * t1155 + t51248 + t51251 + 0.10526802520742363173e2_f64 * t51389 * t11430 - 0.57895126195293126243e3_f64 * t51392 * t11441 + 0.35089341735807877242e1_f64 * t15126 * t11300 - t51399 - t51401 - t51404 + 0.51947577317044391277e2_f64 * t44188 * t4862 + 0.10389515463408878255e3_f64 * t11361 * t15219 + 0.51947577317044391277e2_f64 * t11361 * t15222;
    (t51404, t51411)
}
