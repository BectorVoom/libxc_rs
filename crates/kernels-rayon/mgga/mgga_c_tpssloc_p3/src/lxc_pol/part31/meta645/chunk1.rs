//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1917/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1917(t1520: f64, t254: f64, t23270: f64, t25038: f64, t25039: f64, t4119: f64, t1880: f64, t7488: f64, t87782: f64, t23237: f64, t28276: f64, t6552: f64) -> (f64, f64, f64, f64) {
    let t98279 = t1520 * t254;
    let t98291 = t25038 * t23270 * t25039 * t4119;
    let t98305 = t1880 * t87782 * t7488;
    let t98315 = t6552 * t23237 * t28276;
    (t98279, t98291, t98305, t98315)
}
