//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1589/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1589(t1450: f64, t23059: f64, t22809: f64, t566: f64, t4147: f64, t23087: f64, t9593: f64, t6836: f64, t1921: f64, t6936: f64, t1913: f64, t6951: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t86731 = t23059 * t1450;
    let t86819 = t566 * t22809;
    let t86825 = t23059 * t4147;
    let t86828 = t23087 * t9593;
    let t86839 = t6836 * t566;
    let t86897 = t6936 * t1921;
    let t86903 = t1913 * t6951;
    (t86731, t86819, t86825, t86828, t86839, t86897, t86903)
}
