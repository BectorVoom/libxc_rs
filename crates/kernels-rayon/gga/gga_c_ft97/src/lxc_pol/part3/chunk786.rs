//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 786/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk786(t4551: f64, t492: f64, t8418: f64, t83: f64, t3255: f64, t979: f64, t1852: f64, t1871: f64, t4436: f64, t499: f64, t1882: f64, t4591: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16198 = t4551 * t492;
    let t16199 = t8418 * t16198;
    let t16200 = t83 * t16199;
    let t16203 = t979 * t3255;
    let t16204 = t1852 * t16203;
    let t16205 = t83 * t16204;
    let t16210 = t1871 * t499 * t4436;
    let t16213 = t1882 * t4591;
    (t16199, t16200, t16204, t16205, t16210, t16213)
}
