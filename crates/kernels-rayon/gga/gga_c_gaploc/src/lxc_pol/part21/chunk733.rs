//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 733/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk733(t6907: f64, t912: f64, t587: f64, t2488: f64, t2487: f64, t584: f64, t6715: f64) -> (f64, f64, f64) {
    let t6908 = t912 * t6907;
    let t6909 = t587 * t6908;
    let t6911 = t2488 * t6907;
    let t6912 = t2487 * t6911;
    let t6914 = t584 * t6715;
    (t6909, t6912, t6914)
}
