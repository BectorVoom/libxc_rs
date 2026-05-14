//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1073/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1073<F: Float>(t37891: F, t37903: F, t39816: F, t41555: F, t41573: F, t41574: F, t41575: F, t43248: F, t43251: F, t43256: F, t43259: F, t43262: F, t38568: F, t39846: F, t41576: F, t41577: F, t41578: F, t41584: F, t43266: F, t43269: F, t43271: F, t43273: F, t43275: F, t43277: F) -> (F, F) {
    let t44308 = 0.2600466522016280569e0 * t43248 + 0.10401866088065122276e1 * t43251 + t41555 - 0.85366933852867742946e0 * t37891 - 0.31147743054556651237e-1 * t37903 + 0.23804984598836975487e0 * t39816 - 0.5200933044032561138e0 * t43256 - 0.34672886960217074252e0 * t43259 - 0.52009330440325611378e0 * t43262 - t41573 - t41574 - t41575;
    let t44316 = t41576 - t41577 + 0.54878743191129263322e-2 * t43266 - t38568 - 0.13099107994629972538e-1 * t43269 - 0.26198215989259945076e-1 * t43271 - 0.1047928639570397803e0 * t43273 + 0.43663693315433241794e-2 * t43275 + t41578 + 0.10975748638225852664e-1 * t43277 - 0.16951189180550569635e1 * t39846 - t41584;
    (t44308, t44316)
}
