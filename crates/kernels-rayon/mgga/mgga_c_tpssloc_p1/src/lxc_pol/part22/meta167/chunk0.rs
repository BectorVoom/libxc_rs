//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1010/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1010(t1222: f64, t1731: f64, t1744: f64, t1202: f64, t1743: f64, t225: f64, t4940: f64, t68: f64) -> (f64, f64, f64, f64, f64) {
    let t4957 = t1731 * t1222;
    let t4959 = t1744 * t1222;
    let t4961 = t1202 * t1743;
    let t4964 = t4940 * t225;
    let t4965 = t4964 * t68;
    (t4957, t4959, t4961, t4964, t4965)
}
