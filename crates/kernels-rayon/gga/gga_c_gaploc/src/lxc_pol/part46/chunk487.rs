//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 487/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk487(t1890: f64, t8502: f64, t1980: f64, t2975: f64, t2925: f64, t296: f64, t1: f64, t787: f64, t325: f64, t3039: f64, t783: f64, t1022: f64, t5750: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8503 = t1890 * t8502;
    let t8516 = t1980 * t2975;
    let t8519 = t296 * t2925;
    let t8520 = t8519 * t1;
    let t8521 = t787 * t8520;
    let t8528 = t325 * t2925;
    let t8556 = t3039 * t783;
    let t8604 = t5750 * t1022;
    (t8503, t8516, t8521, t8528, t8556, t8604)
}
