//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 713/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk713(t7021: f64, t840: f64, t882: f64, t1882: f64, t7047: f64, t7126: f64, t7051: f64, t28857: f64, t296: f64, t7059: f64, t2749: f64, t7105: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t29378 = t840 * t882 * t7021;
    let t29383 = t1882 * t7047;
    let t29385 = t1882 * t7126;
    let t29387 = t1882 * t7051;
    let t29389 = t296 * t28857;
    let t29392 = t1882 * t7059;
    let t29396 = t840 * t2749 * t7105;
    (t29378, t29383, t29385, t29387, t29389, t29392, t29396)
}
