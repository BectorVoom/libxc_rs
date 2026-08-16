//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1157/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1157(t148333: f64, t148377: f64, t148432: f64, t148488: f64, t148541: f64, t148589: f64, t148638: f64, t148683: f64, t1882: f64, t35169: f64, t27191: f64, t5935: f64) -> (f64, f64, f64) {
    let t148686 = t148333 + t148377 + t148432 + t148488 + t148541 + t148589 + t148638 + t148683;
    let t148692 = t1882 * t35169;
    let t148703 = t5935 * t27191;
    (t148686, t148692, t148703)
}
