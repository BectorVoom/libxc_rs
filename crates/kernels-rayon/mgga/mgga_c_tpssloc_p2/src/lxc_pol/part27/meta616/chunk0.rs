//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2093/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2093(t10984: f64, t6717: f64, t1036: f64, t23557: f64, t1933: f64, t1937: f64, t2250: f64, t3200: f64, t83015: f64, t1030: f64, t1058: f64, t3068: f64, sigma0: f64) -> (f64, f64, f64, f64, f64) {
    let t83167 = t6717 * t10984;
    let t83172 = t23557 * t1036;
    let t83206 = t1933 * t2250 * t1937;
    let t83215 = t3200 * t83015;
    let t83220 = t1058 * sigma0 * t1030 * t3068;
    (t83167, t83172, t83206, t83215, t83220)
}
