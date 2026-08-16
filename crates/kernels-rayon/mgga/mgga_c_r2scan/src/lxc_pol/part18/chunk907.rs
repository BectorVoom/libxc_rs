//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 907/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk907(t4721: f64, t4964: f64, t4967: f64, t4972: f64, t4975: f64, t4979: f64, t4981: f64, t6961: f64, t8555: f64, t8556: f64, t8559: f64, t8560: f64) -> f64 {
    let t9794 = t8555 - t4721 + t4964 - t4967 - t8556 - t4972 + t4975 - t8559 - t8560 + t4979 - t4981 - t6961;
    t9794
}
