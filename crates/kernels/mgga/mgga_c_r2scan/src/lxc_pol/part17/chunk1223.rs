//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1223/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1223<F: Float>(t39558: F, t41435: F, t43115: F, t43117: F, t43120: F, t43123: F, t43126: F, t43130: F, t43133: F, t43135: F, t43138: F, t43141: F) -> F {
    let t44242 = F::cast_from(0.10975748638225852664e0_f64) * t43115 - t41435 - F::cast_from(0.10975748638225852664e-1_f64) * t43117 - F::cast_from(0.90044238659382329742e0_f64) * t39558 - F::cast_from(0.26198215989259945077e-1_f64) * t43120 + F::cast_from(0.87327386630866483588e-2_f64) * t43123 + F::cast_from(0.43663693315433241794e-2_f64) * t43126 + F::cast_from(0.43663693315433241794e-2_f64) * t43130 - F::cast_from(0.27944763721877274748e0_f64) * t43133 + F::cast_from(0.5200933044032561138e0_f64) * t43135 + F::cast_from(0.2600466522016280569e0_f64) * t43138 + F::cast_from(0.52009330440325611378e0_f64) * t43141;
    t44242
}
