//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 920/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk920(t1443: f64, t676: f64, t6907: f64, t737: f64, t24737: f64, t53798: f64, t1456: f64, t9895: f64, t6154: f64, t7021: f64, t880: f64, t1253: f64, t6260: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t110751 = t676 * t1443;
    let t110950 = t737 * t6907;
    let t111089 = t53798 * t24737;
    let t111330 = t9895 * t1456;
    let t111518 = t737 * t6154;
    let t111668 = t7021 * t880;
    let t111711 = t6260 * t1253;
    (t110751, t110950, t111089, t111330, t111518, t111668, t111711)
}
