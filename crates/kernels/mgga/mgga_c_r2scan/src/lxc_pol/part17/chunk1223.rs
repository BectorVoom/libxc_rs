//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1223/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1223<F: Float>(t39558: F, t41435: F, t43115: F, t43117: F, t43120: F, t43123: F, t43126: F, t43130: F, t43133: F, t43135: F, t43138: F, t43141: F) -> F {
    let t44242 = F::new(0.10975748638225852664e0) * t43115 - t41435 - F::new(0.10975748638225852664e-1) * t43117 - F::new(0.90044238659382329742e0) * t39558 - F::new(0.26198215989259945077e-1) * t43120 + F::new(0.87327386630866483588e-2) * t43123 + F::new(0.43663693315433241794e-2) * t43126 + F::new(0.43663693315433241794e-2) * t43130 - F::new(0.27944763721877274748e0) * t43133 + F::new(0.5200933044032561138e0) * t43135 + F::new(0.2600466522016280569e0) * t43138 + F::new(0.52009330440325611378e0) * t43141;
    t44242
}
