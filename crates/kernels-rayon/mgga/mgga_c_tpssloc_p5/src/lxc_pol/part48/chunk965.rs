//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 965/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk965(t23035: f64, t23241: f64, t31366: f64, t114790: f64, t23164: f64, t6555: f64, t1880: f64, t23237: f64, t31419: f64, t2047: f64, t212: f64, t23171: f64, t6554: f64) -> (f64, f64, f64, f64) {
    let t114913 = t23035 * t31366 * t23241;
    let t114916 = t23164 * t114790 * t6555;
    let t114926 = t1880 * t23237 * t31419;
    let t114932 = t23171 * t212 * t2047 * t6554;
    (t114913, t114916, t114926, t114932)
}
