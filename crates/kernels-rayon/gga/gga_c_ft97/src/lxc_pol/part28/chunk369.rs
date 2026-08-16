//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 369/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk369(t363: f64, t5691: f64, t1564: f64, t446: f64, t432: f64, t5507: f64, t28: f64, t89: f64, t370: f64, t5617: f64, t27: f64, t5669: f64, t5673: f64, t5678: f64, t5682: f64, t5686: f64, t5690: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5692 = t5691 * t363;
    let t5693 = t1564 * t5692;
    let t5694 = t446 * t5693;
    let t5696 = t5507 * t432;
    let t5697 = t28 * t5696;
    let t5698 = t89 * t5697;
    let t5700 = t370 * t5617;
    let t5702 = t89 * t27 * t5700;
    let t5704 = t5669 / 12.0_f64 + t5673 + t5678 / 18.0_f64 + t5682 / 3.0_f64 - t5686 / 6.0_f64 + t5690 + t5694 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t5698 - t5702 / 3.0_f64;
    (t5692, t5693, t5694, t5696, t5697, t5698, t5700, t5702, t5704)
}
