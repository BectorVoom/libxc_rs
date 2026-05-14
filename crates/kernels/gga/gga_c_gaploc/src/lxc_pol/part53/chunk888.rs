//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 888/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk888<F: Float>(t42730: F, t42733: F, t42737: F, t42739: F, t42742: F, t42756: F, t42763: F, t42767: F, t42771: F, t42772: F, t42773: F, t42790: F, t42793: F, t42795: F, t42797: F, t46979: F, t46980: F, t46991: F) -> (F,) {
    let t50962 = -t42730 + t42733 - t46979 - 0.22764005308540919679e0 * t46980 - t42737 + t42739 + t42742 - t42756 + t46991 + t42763 + t42767 - t42771 - t42772 + t42773 + t42790 + t42793 - t42795 - t42797;
    (t50962,)
}
