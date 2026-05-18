//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 213/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk213<F: Float>(t598: F, t610: F, t186: F, t185: F, t202: F, t209: F, t184: F) -> (F, F, F, F, F) {
    let t611 = t598 * t610;
    let t612 = t186 * t611;
    let t614 = F::new(2.0) / F::new(15.0) * t185 * t612;
    let t615 = t202 * t209;
    let t616 = t615 * t184;
    (t611, t612, t614, t615, t616)
}
