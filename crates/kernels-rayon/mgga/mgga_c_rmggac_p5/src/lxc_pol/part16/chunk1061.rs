//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1061/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1061(t10262: f64, t10263: f64, t42384: f64, t42385: f64, t42386: f64, t42390: f64, t42391: f64, t42392: f64, t42393: f64, t42394: f64, t42395: f64, t10269: f64, t10272: f64, t10273: f64, t10274: f64, t37089: f64, t37096: f64, t37099: f64, t37100: f64, t8057: f64, t8069: f64, t8070: f64) -> (f64, f64) {
    let t48105 = -t42384 + t42385 + t10262 - t42386 - t10263 + t42390 + t42391 - t42392 + t42393 + t42394 - t42395;
    let t48111 = -t8057 + t37089 - t37096 - t8069 - t8070 + t37099 - t37100 + t10269 + t10272 + t10273 + t10274;
    (t48105, t48111)
}
