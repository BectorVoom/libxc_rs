//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 920/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk920<F: Float>(t10262: F, t10263: F, t42384: F, t42385: F, t42386: F, t42390: F, t42391: F, t42392: F, t42393: F, t42394: F, t42395: F, t10269: F, t10272: F, t10273: F, t10274: F, t37089: F, t37096: F, t37099: F, t37100: F, t8057: F, t8069: F, t8070: F) -> (F, F) {
    let t48105 = -t42384 + t42385 + t10262 - t42386 - t10263 + t42390 + t42391 - t42392 + t42393 + t42394 - t42395;
    let t48111 = -t8057 + t37089 - t37096 - t8069 - t8070 + t37099 - t37100 + t10269 + t10272 + t10273 + t10274;
    (t48105, t48111)
}
