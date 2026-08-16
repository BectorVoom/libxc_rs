//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1466/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1466(t120973: f64, t120975: f64, t120979: f64, t120986: f64, t120991: f64, t120993: f64, t24932: f64, t27150: f64, t27171: f64, t27293: f64, t31832: f64, t32350: f64, t4077: f64, t652: f64, t7042: f64, t7056: f64, t7266: f64, t7802: f64, t7806: f64, t7904: f64, t8103: f64) -> f64 {
    let t124918 = -2.0_f64 * t652 * t7056 * t8103 - 2.0_f64 * t24932 * t7802 - 2.0_f64 * t24932 * t7806 - 2.0_f64 * t27150 * t7266 - 2.0_f64 * t27171 * t7266 - 2.0_f64 * t27293 * t7042 + 3.0_f64 * t31832 * t7904 - 2.0_f64 * t32350 * t4077 - t120973 + t120975 - t120979 - t120986 + t120991 - t120993;
    t124918
}
