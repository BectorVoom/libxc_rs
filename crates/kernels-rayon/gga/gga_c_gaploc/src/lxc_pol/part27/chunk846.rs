//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 846/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk846(t549: f64, t7981: f64, t1397: f64, t2897: f64, t1402: f64, t2783: f64, t1359: f64, t986: f64, t1415: f64, t107: f64, t7887: f64, t544: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8226 = t549 * t7981;
    let t8229 = t1397 * t2897;
    let t8233 = t1402 * t2783;
    let t8237 = t1359 * t986;
    let t8238 = t1415 * t8237;
    let t8247 = t7887 * t107;
    let t8248 = t544 * t8247;
    (t8226, t8229, t8233, t8237, t8238, t8247, t8248)
}
