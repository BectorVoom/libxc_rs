//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 735/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk735(t11785: f64, t337: f64, t2121: f64, t2133: f64, t3916: f64, t3717: f64, t5: f64, t2147: f64, t2164: f64, t3832: f64, t2142: f64, t3783: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11786 = t337 * t11785;
    let t11787 = t2121 * t11786;
    let t11794 = t3916 * t2133;
    let t11806 = t5 * t3717;
    let t11807 = t337 * t11806;
    let t11808 = t2147 * t11807;
    let t11811 = t2164 * t3832;
    let t11817 = t3783 * t2142;
    (t11786, t11787, t11794, t11806, t11807, t11808, t11811, t11817)
}
