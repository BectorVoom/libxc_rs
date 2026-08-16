//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 532/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk532(t169: f64, t2754: f64, t172: f64, t452: f64, t158: f64, t986: f64, t123: f64, t488: f64, t555: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2755 = t2754 * t169;
    let t2756 = t2755 * t172;
    let t2757 = t452 * t2756;
    let t2760 = t158 * t986;
    let t2761 = t2760 * t123;
    let t2762 = t2761 * t488;
    let t2765 = t555 * t986;
    (t2755, t2756, t2757, t2760, t2761, t2762, t2765)
}
