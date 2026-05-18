//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1084/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1084<F: Float>(t42786: F, t42790: F, t42793: F, t42795: F, t42797: F, t42799: F, t42802: F, t42803: F, t42804: F, t42806: F, t42808: F, t1063: F, t38267: F, t894: F) -> (F, F) {
    let t46999 = -F::new(0.17073003981405689759e0) * t42786 + t42790 + t42793 - t42795 - t42797 - t42799 - t42802 - t42803 + t42804 + t42806 - t42808;
    let t47001 = t1063 * t894 * t38267;
    (t46999, t47001)
}
