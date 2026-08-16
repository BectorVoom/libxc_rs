//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1356/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1356(t119824: f64, t119826: f64, t119830: f64, t120664: f64, t120899: f64, t120900: f64, t120907: f64, t120910: f64, t120912: f64, t22461: f64, t24999: f64, t26103: f64, t26559: f64, t27180: f64, t27219: f64, t6517: f64, t7061: f64, t7806: f64) -> f64 {
    let t120921 = 2.0_f64 * t120664 * t26559 - 2.0_f64 * t22461 * t7806 - 2.0_f64 * t24999 * t7061 - 2.0_f64 * t26103 * t7806 - 2.0_f64 * t27180 * t6517 - 2.0_f64 * t27219 * t6517 - t119824 - t119826 - t119830 - t120899 - t120900 + t120907 - t120910 - t120912;
    t120921
}
