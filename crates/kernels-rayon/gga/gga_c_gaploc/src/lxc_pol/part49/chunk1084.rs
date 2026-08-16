//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1084/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1084(t42786: f64, t42790: f64, t42793: f64, t42795: f64, t42797: f64, t42799: f64, t42802: f64, t42803: f64, t42804: f64, t42806: f64, t42808: f64, t1063: f64, t38267: f64, t894: f64) -> (f64, f64) {
    let t46999 = -0.17073003981405689759e0_f64 * t42786 + t42790 + t42793 - t42795 - t42797 - t42799 - t42802 - t42803 + t42804 + t42806 - t42808;
    let t47001 = t1063 * t894 * t38267;
    (t46999, t47001)
}
