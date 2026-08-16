//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 832/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk832(t7755: f64, t7756: f64, t7759: f64, t7761: f64, t8252: f64, t8253: f64, t8254: f64, t8257: f64, t8904: f64, t8909: f64, t8913: f64, t8917: f64, t8921: f64, t8925: f64, t8930: f64) -> f64 {
    let t9345 = 0.10718504529517434243e-2_f64 * t8904 + 0.42874018118069736972e-3_f64 * t8909 - 0.21437009059034868486e-3_f64 * t8913 - 0.916875e-1_f64 * t8917 - 0.4584375e-1_f64 * t8921 - 0.4584375e-1_f64 * t8925 - 0.4584375e-1_f64 * t8930 - t8252 - t8253 + t8254 + t8257 - t7755 + 0.6431102717710460546e-2_f64 * t7756 + t7759 - t7761;
    t9345
}
