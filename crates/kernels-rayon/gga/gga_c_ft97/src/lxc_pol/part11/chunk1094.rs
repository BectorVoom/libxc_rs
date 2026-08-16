//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1094/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1094(t8392: f64, t9850: f64, t761: f64, t9577: f64, t10094: f64, t9788: f64, t2596: f64, t8232: f64, t1882: f64, t9835: f64, t2591: f64, t3281: f64, t726: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42853 = t8392 * t9850;
    let t42859 = t761 * t9577;
    let t42864 = t8392 * t10094;
    let t42874 = t8392 * t9788;
    let t42879 = t8232 * t2596;
    let t42881 = t1882 * t9835;
    let t42884 = t8232 * t2591;
    let t42894 = t3281 * t726;
    (t42853, t42859, t42864, t42874, t42879, t42881, t42884, t42894)
}
