//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 676/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk676(t6856: f64, t2250: f64, t825: f64, t126: f64, t2723: f64, t442: f64, t2669: f64, t2206: f64, t768: f64) -> (f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t6857 = t6856 * pi;
    let t6924 = t2250 * t825;
    let t6925 = t6924 * t126;
    let t6927 = t2723 * t442;
    let t6935 = t2669 * t442;
    let t6939 = t768 * t2206;
    (t6857, t6924, t6925, t6927, t6935, t6939)
}
