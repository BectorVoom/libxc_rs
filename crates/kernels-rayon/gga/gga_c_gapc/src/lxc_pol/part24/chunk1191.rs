//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1191/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1191(t11455: f64, t9325: f64, t11312: f64, t4940: f64, t11320: f64, t1875: f64, t5190: f64, t1765: f64, t3670: f64, t11391: f64, t3163: f64, t128: f64, t203: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34851 = t11455 * t9325;
    let t34853 = t11312 * t4940;
    let t34856 = t1875 * t11320 * t5190;
    let t34858 = t3670 * t1765;
    let t34860 = t11391 * t3163;
    let t34863 = t203 * t128;
    (t34851, t34853, t34856, t34858, t34860, t34863)
}
