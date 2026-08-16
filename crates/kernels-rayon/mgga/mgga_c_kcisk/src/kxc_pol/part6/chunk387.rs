//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 387/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk387(t2507: f64, t673: f64, t716: f64, t720: f64, t415: f64, t1876: f64, t1877: f64, t2063: f64, t1882: f64, t2372: f64, t706: f64, t1887: f64, t2487: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2508 = t673 * t2507;
    let t2509 = t2508 * t716;
    let t2510 = t2509 * t720;
    let t2511 = t415 * t2510;
    let t2514 = t1876 * t1877 * t2063;
    let t2517 = t1882 * t2372;
    let t2518 = t706 * t2517;
    let t2521 = t1887 * t2487;
    (t2509, t2510, t2511, t2514, t2517, t2518, t2521)
}
