//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1000/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1000<F: Float>(t20233: F, t5676: F, t3935: F, t13607: F, t403: F, t5671: F, t1324: F, t13795: F, t13913: F, t13919: F, t13924: F, t20084: F, t20226: F, t20230: F, t2170: F, t3970: F, t3990: F, t6157: F, t6213: F) -> (F, F) {
    let t20234 = t20233 * t5676;
    let t20236 = 0.2398771828823642295e-1 * t3935 * t20234;
    let t20237 = t13607 * t403;
    let t20238 = t20237 * t5671;
    let t20240 = 0.159918121921576153e-1 * t3935 * t20238;
    let t20241 = 0.59969295720591057377e-2 * t13913 + 0.79959060960788076502e-2 * t13919 - 0.11993859144118211475e-1 * t13924 - 0.5397236614853195164e-1 * t13795 * t2170 - 0.5397236614853195164e-1 * t6157 * t3990 + 0.28785261945883707542e0 * t3970 * t6213 - t20226 + 0.28785261945883707542e0 * t20084 * t1324 + 0.71963154864709268852e-1 * t3935 * t20230 - t20236 + t20240;
    (t20237, t20241)
}
