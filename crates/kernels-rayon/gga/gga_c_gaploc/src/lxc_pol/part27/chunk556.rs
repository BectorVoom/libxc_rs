//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 556/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk556(t169: f64, t2925: f64, t299: f64, t706: f64, t1022: f64, t296: f64, t123: f64, t734: f64, t795: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2926 = t2925 * t169;
    let t2927 = t2926 * t299;
    let t2928 = t706 * t2927;
    let t2931 = t296 * t1022;
    let t2932 = t2931 * t123;
    let t2933 = t2932 * t734;
    let t2936 = t795 * t1022;
    (t2926, t2927, t2928, t2931, t2932, t2933, t2936)
}
