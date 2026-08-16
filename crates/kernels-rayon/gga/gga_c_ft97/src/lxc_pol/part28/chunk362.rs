//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 362/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk362(t110: f64, t1871: f64, t5635: f64, t1307: f64, t452: f64, t499: f64, t492: f64, t488: f64, t5617: f64, t1328: f64, t376: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5637 = t1871 * t110 * t5635;
    let t5641 = t452 * t499 * t1307;
    let t5644 = t1307 * t492;
    let t5646 = t452 * t488 * t5644;
    let t5650 = t452 * t110 * t5617;
    let t5655 = t89 * t376 * t1328 / 9.0_f64;
    (t5637, t5641, t5644, t5646, t5650, t5655)
}
