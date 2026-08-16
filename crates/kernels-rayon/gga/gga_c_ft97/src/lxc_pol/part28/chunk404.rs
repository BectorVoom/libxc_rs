//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 404/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk404(t574: f64, t5947: f64, t605: f64, t1384: f64, t2142: f64, t144: f64, t609: f64) -> (f64, f64, f64) {
    let t5949 = t574 * t605 * t5947;
    let t5952 = t2142 * t1384;
    let t5953 = t144 * t5952;
    let t5956 = t1384 * t609;
    (t5949, t5953, t5956)
}
