//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1190/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1190(t81281: f64, t12023: f64, t12033: f64, t1375: f64, t1385: f64, t2092: f64, t24138: f64, t24139: f64, t24147: f64, t3758: f64, t3887: f64, t39916: f64, t7194: f64, t7214: f64, t81264: f64, t81267: f64, t81272: f64, t81284: f64) -> f64 {
    let t84423 = 0.19739208802178717238e0_f64 * t81281;
    let t84429 = -3.0_f64 * t3758 * t24139 - 6.0_f64 * t7194 * t12023 + 0.15626873635058151147e0_f64 * t81264 - 3.0_f64 * t12033 * t7214 + 6.0_f64 * t1375 * t3887 * t24138 * t1385 + 0.49348022005446793095e-1_f64 * t81267 - 0.19739208802178717238e0_f64 * t81272 + t84423 + 12.0_f64 * t3758 * t24147 - 3.0_f64 * t39916 * t2092 + 0.9869604401089358619e-1_f64 * t81284;
    t84429
}
