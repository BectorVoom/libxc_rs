//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 978/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk978(t3443: f64, t7514: f64, t17819: f64, t1820: f64, t3410: f64, t16621: f64, t3414: f64, t587: f64, t11190: f64, t2000: f64, t10609: f64, t20: f64, t2004: f64) -> (f64, f64, f64, f64, f64) {
    let t33105 = t7514 * t3443;
    let t33149 = t1820 * t17819 * t3410;
    let t33152 = t587 * t16621 * t3414;
    let t33193 = t11190 * t2000;
    let t33196 = t10609 * t20 * t2004;
    (t33105, t33149, t33152, t33193, t33196)
}
