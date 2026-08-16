//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 642/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk642(t375: f64, t4669: f64, t89: f64, t160: f64, t4714: f64, t1882: f64, t4726: f64, t4824: f64, t8392: f64, t2178: f64, t4724: f64, t4668: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16928 = t89 * t375 * t4669;
    let t16963 = t160 * t4714;
    let t16969 = t1882 * t4726;
    let t16986 = t8392 * t4824;
    let t17016 = t2178 * t4724;
    let t17021 = t160 * t4668;
    (t16928, t16963, t16969, t16986, t17016, t17021)
}
