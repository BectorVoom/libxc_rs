//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 535/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk535(t1609: f64, t8: f64, t1613: f64, t5585: f64, t6010: f64, t681: f64, t1424: f64, t7514: f64, t2371: f64, t6061: f64) -> (f64, f64, f64, f64, f64) {
    let t22532 = t8 * t1609;
    let t22794 = t5585 * t1613;
    let t24178 = t681 * t6010;
    let t24181 = t7514 * t1424;
    let t24191 = t2371 * t6061;
    (t22532, t22794, t24178, t24181, t24191)
}
