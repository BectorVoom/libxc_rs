//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1167/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1167<F: Float>(t100930: F, t100933: F, t100936: F, t100940: F, t100942: F, t100945: F, t100950: F, t100952: F, t100954: F, t100957: F, t101612: F, t101713: F, t101716: F, t101718: F, t101720: F, t101723: F, t101730: F, t101732: F, t101734: F) -> (F,) {
    let t101740 = -t100930 - t100933 - t100936 - t100940 + t100942 + t100945 - t100950 - t100952 + t100954 - t100957 + t101612 + t101713 - t101716 - t101718 - t101720 - t101723 + t101730 + t101732 - t101734;
    (t101740,)
}
