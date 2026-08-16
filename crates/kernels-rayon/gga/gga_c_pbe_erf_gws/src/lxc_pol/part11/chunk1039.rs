//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1039/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1039(t13223: f64, t21825: f64, t13184: f64, t8801: f64, t13624: f64, t9270: f64, t13189: f64, t353: f64, t3886: f64, t859: f64, t8787: f64, t13645: f64, t2246: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43903 = t21825 * t13223;
    let t43917 = t8801 * t13184;
    let t43919 = t9270 * t13624;
    let t43942 = t8801 * t13189;
    let t43983 = t859 * t353 * t8787 * t3886;
    let t44019 = t2246 * t13645;
    (t43903, t43917, t43919, t43942, t43983, t44019)
}
