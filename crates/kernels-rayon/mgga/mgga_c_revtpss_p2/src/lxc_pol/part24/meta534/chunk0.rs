//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1573/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1573(t22837: f64, t9962: f64, t22860: f64, t47194: f64, t22849: f64, t3957: f64, t13790: f64, t22020: f64, t2661: f64, t9934: f64, t177: f64, t22789: f64, t762: f64) -> (f64, f64, f64, f64, f64) {
    let t85839 = t9962 * t22837;
    let t85865 = t47194 * t22860;
    let t85873 = t3957 * t22849;
    let t85885 = t2661 * t9934 * t22020 * t13790;
    let t85895 = t22789 * t177 * t762;
    (t85839, t85865, t85873, t85885, t85895)
}
