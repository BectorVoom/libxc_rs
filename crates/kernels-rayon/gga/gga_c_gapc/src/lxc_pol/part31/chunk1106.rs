//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1106/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1106(t126: f64, t17890: f64, t276: f64, t314: f64, t442: f64, t2206: f64, t2250: f64, t103: f64, t2723: f64, t1087: f64, t2404: f64, t1: f64, t6852: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t22949 = t276 * t17890 * t126;
    let t22954 = t314 * pi * t442;
    let t22970 = t2250 * t2206;
    let t22971 = t22970 * t126;
    let t22973 = t2723 * t103;
    let t23104 = t1087 * t2404;
    let t23132 = t6852 * t1;
    (t22949, t22954, t22970, t22971, t22973, t23104, t23132)
}
