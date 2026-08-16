//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1147/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1147(t27142: f64, t3052: f64, t32924: f64, t9073: f64, t148288: f64, t446: f64, t34822: f64, t558: f64, t9432: f64, t1369: f64, t147590: f64, t28: f64, t586: f64) -> (f64, f64, f64, f64) {
    let t148545 = t27142 * t9073 * t32924 * t3052;
    let t148551 = t446 * t9073 * t148288;
    let t148555 = t446 * t9432 * t34822 * t558;
    let t148559 = t1369 * t28 * t586 * t147590;
    (t148545, t148551, t148555, t148559)
}
