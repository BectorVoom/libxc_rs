//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2622/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2622(t11786: f64, t5024: f64, t3509: f64, t607: f64, t3032: f64, t52434: f64, t3505: f64, t1090: f64, t11678: f64, t1174: f64, t11855: f64, t1196: f64, t15525: f64, t15591: f64, t3252: f64, t3496: f64, t3511: f64, t3577: f64, t3578: f64, t45222: f64, t45224: f64, t45227: f64, t45872: f64, t4728: f64, t5002: f64, t5012: f64, t974: f64) -> (f64, f64, f64) {
    let t53360 = t5024 * t11786;
    let t53366 = t3509 * t607;
    let t53371 = t52434 * t3032;
    let t53372 = t53371 * t3505;
    let t53377 = t5002 * t11855 / 3072.0_f64 - t1174 * t974 * t1196 * t45872 / 288.0_f64 - t3577 * t3578 * t15525 * t1090 / 1536.0_f64 - t45222 / 144.0_f64 - t45224 / 4608.0_f64 + t45227 / 216.0_f64 - 5.0_f64 / 1296.0_f64 * t53360 - t3577 * t3578 * t5012 * t3252 / 1536.0_f64 - t11678 * t3578 * t4728 * t53366 / 384.0_f64 + t53372 * t3511 / 512.0_f64 + t15591 * t3496 / 1024.0_f64;
    (t53366, t53371, t53377)
}
