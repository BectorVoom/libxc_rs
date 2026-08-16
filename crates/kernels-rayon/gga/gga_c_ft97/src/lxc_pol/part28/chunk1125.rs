//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1125/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1125(t35087: f64, t8392: f64, t23478: f64, t6718: f64, t23997: f64, t26523: f64, t2179: f64, t34947: f64, t609: f64, t1882: f64, t35122: f64, t35203: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t148194 = t8392 * t35087;
    let t148196 = t23478 * t6718;
    let t148205 = t23997 * t26523;
    let t148210 = t2179 * t34947 * t609;
    let t148219 = t1882 * t35122;
    let t148221 = t1882 * t35203;
    (t148194, t148196, t148205, t148210, t148219, t148221)
}
