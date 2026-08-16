//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2344/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2344(t22550: f64, t7974: f64, t2109: f64, t90247: f64, t1419: f64, t2274: f64, t12606: f64, t12648: f64, t12652: f64, t14165: f64, t1860: f64, t1864: f64, t2108: f64, t2110: f64, t2244: f64, t2250: f64, t22549: f64, t24498: f64, t24505: f64, t24508: f64, t26009: f64, t26028: f64, t27303: f64, t27356: f64, t27364: f64, t27365: f64, t608: f64, t6486: f64, t6509: f64, t67: f64, t7251: f64, t7256: f64, t7259: f64, t7428: f64, t83803: f64, t85539: f64, t90121: f64, t9239: f64) -> f64 {
    let t96135 = t7974 * t22550;
    let t96138 = t2109 * t90247;
    let t96157 = t1419 * t2274;
    let t96180 = -10.0_f64 / 3.0_f64 * t22549 * t96135 - 10.0_f64 / 3.0_f64 * t22549 * t96138 + 20.0_f64 * t9239 * t608 * t2108 * t26009 - t90121 * t2110 / 6.0_f64 - t26028 * t7256 / 3.0_f64 - t26028 * t7259 / 3.0_f64 - t7428 * t24505 / 6.0_f64 - t7428 * t24508 / 3.0_f64 - t6486 * t27365 / 3.0_f64 - t1860 * (-20.0_f64 / 27.0_f64 * t96157 * t2244 + 20.0_f64 / 9.0_f64 * t27356 * t2250 + 5.0_f64 / 108.0_f64 * t85539 * t14165 + 5.0_f64 / 9.0_f64 * t24498 * t12652 + 5.0_f64 / 18.0_f64 * t24498 * t12648 - 5.0_f64 / 6.0_f64 * t7251 * t12606 + t83803) * t67 * t1864 / 6.0_f64 - t1860 * t27364 * t6509 / 3.0_f64 - t6486 * t27303 / 3.0_f64;
    t96180
}
