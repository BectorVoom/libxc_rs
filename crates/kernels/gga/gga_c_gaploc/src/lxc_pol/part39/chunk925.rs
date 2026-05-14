//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 925/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk925<F: Float>(t42756: F, t42759: F, t42763: F, t42767: F, t42771: F, t42772: F, t42773: F, t42774: F, t42778: F, t42782: F, t46991: F, t42786: F, t42790: F, t42793: F, t42795: F, t42797: F, t42799: F, t42802: F, t42803: F, t42804: F, t42806: F, t42808: F) -> (F, F) {
    let t46996 = -t42756 + t46991 + 0.28455006635676149599e-1 * t42759 + t42763 + t42767 - t42771 - t42772 + t42773 - 0.15808337019820083111e-2 * t42774 - 0.19918504644973304719e0 * t42778 + 0.34146007962811379518e0 * t42782;
    let t46999 = -0.17073003981405689759e0 * t42786 + t42790 + t42793 - t42795 - t42797 - t42799 - t42802 - t42803 + t42804 + t42806 - t42808;
    (t46996, t46999)
}
