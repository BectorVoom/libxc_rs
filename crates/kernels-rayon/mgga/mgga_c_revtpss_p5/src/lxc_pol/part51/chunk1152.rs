//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1152/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1152(t119849: f64, t4500: f64, t120090: f64, t119891: f64, t14686: f64, t1579: f64, t119968: f64, t119836: f64, t119888: f64, t27279: f64, t31854: f64, t33711: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t126368 = t119849 * t4500;
    let t126370 = t120090 * t4500;
    let t126375 = t14686 * t119891 * t1579;
    let t126376 = t119968 * t126375;
    let t126378 = t119836 * t126375;
    let t126380 = t119888 * t27279;
    let t126384 = t33711 * t31854;
    (t126368, t126370, t126376, t126378, t126380, t126384)
}
