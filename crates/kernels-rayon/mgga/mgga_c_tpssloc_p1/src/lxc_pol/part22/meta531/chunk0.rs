//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2004/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2004(t2509: f64, t2512: f64, t745: f64, t9711: f64, t1294: f64, t2504: f64, t9493: f64, t2369: f64, t9489: f64, t116: f64, t4: f64, t126: f64, t268: f64, t8705: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39259 = t2509 * t9711 * t2512 * t745;
    let t39261 = 0.69263436422725855036e2_f64 * t1294 * t39259;
    let t39263 = t9493 * t2504;
    let t39264 = t9489 * t2369 * t39263;
    let t39266 = 0.61524113149298439947e4_f64 * t1294 * t39264;
    let t39267 = t116 * t4;
    let t39273 = 1.0_f64 / t126 / t39267 * t116 * t8705 * t268 / 48.0_f64;
    (t39259, t39261, t39263, t39264, t39266, t39267, t39273)
}
