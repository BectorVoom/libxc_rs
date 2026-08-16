//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 826/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk826(t1391: f64, t2185: f64, t5860: f64, t23997: f64, t5956: f64, t144: f64, t379: f64, t569: f64, t7414: f64, t574: f64, t5935: f64, t5947: f64) -> (f64, f64, f64, f64, f64) {
    let t33176 = t2185 * t1391 * t5860;
    let t33179 = t23997 * t5956;
    let t33180 = t144 * t33179;
    let t33184 = t569 * t7414 * t379;
    let t33188 = t574 * t5935 * t5947;
    (t33176, t33179, t33180, t33184, t33188)
}
