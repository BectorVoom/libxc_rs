//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1280/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1280(t11391: f64, t677: f64, t11412: f64, t169: f64, t4043: f64, t8960: f64, t11587: f64, t27940: f64, t2993: f64, t11604: f64, t27868: f64, t33748: f64, t8843: f64) -> (f64, f64, f64, f64, f64) {
    let t35259 = t11391 * t677;
    let t35263 = t169 * t11412 * t4043 * t8960;
    let t35266 = t2993 * t11587 * t27940;
    let t35269 = t11604 * t27868;
    let t35272 = t2993 * t33748 * t8843;
    (t35259, t35263, t35266, t35269, t35272)
}
