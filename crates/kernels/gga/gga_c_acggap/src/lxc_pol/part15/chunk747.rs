//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 747/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk747<F: Float>(t7802: F, t7805: F, t7849: F, t7853: F, t7862: F, t394: F, t633: F) -> (F, F, F, F, F, F) {
    let t8276 = F::new(0.31448092289604152069e-3) * t7802;
    let t8278 = F::new(0.41930789719472202758e-3) * t7805;
    let t8291 = F::new(77.0) / F::new(864.0) * t7849;
    let t8292 = F::new(35.0) / F::new(216.0) * t7853;
    let t8294 = t7862 / F::new(192.0);
    let t8306 = t394 * t633;
    (t8276, t8278, t8291, t8292, t8294, t8306)
}
