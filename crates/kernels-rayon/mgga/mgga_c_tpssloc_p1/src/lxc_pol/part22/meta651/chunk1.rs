//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2193/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2193(t13133: f64, t4101: f64, t16616: f64, t2371: f64, t17083: f64, t225: f64, t16805: f64, t68: f64, t16752: f64, t252: f64, t13396: f64, t1499: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t58052 = t13133 * t4101;
    let t58057 = t16616 * t2371;
    let t58143 = t17083 * t225;
    let t58181 = t16805 * t68;
    let t58262 = t252 * t16752;
    let t58313 = t1499 * t13396;
    (t58052, t58057, t58143, t58181, t58262, t58313)
}
