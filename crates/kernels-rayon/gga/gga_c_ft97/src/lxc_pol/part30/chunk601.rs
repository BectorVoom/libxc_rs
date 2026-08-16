//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 601/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk601(t2404: f64, t743: f64, t24519: f64, t3886: f64, t6118: f64, t684: f64, t6852: f64, t24432: f64, t24438: f64, t6878: f64, t24437: f64, t747: f64, t992: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27762 = t2404 * t743;
    let t27763 = t24519 * t3886;
    let t27764 = t27762 * t27763;
    let t27765 = t6118 * t27764;
    let t27767 = t6852 * t684;
    let t27768 = t24432 * t27767;
    let t27769 = t6118 * t27768;
    let t27772 = t24438 * t6878 * t684;
    let t27773 = t24437 * t27772;
    let t27775 = t992 * t747;
    (t27762, t27763, t27765, t27767, t27769, t27773, t27775)
}
