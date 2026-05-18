//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 521/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk521<F: Float>(t3507: F, t492: F, t1506: F, t1505: F, t1512: F, t1504: F, t497: F, t1414: F, t381: F, t79: F, t3742: F, t3784: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4223 = t3507 * t492;
    let t4224 = t4223 * t1506;
    let t4226 = t1512 * t1505;
    let t4227 = t1504 * t4226;
    let t4229 = t492 * t497;
    let t4230 = t1414 * t4229;
    let t4231 = t79 * t381;
    let t4232 = t4231 * t3742;
    let t4233 = t4230 * t4232;
    let t4235 = t3784 * t492;
    (t4223, t4224, t4226, t4227, t4229, t4230, t4231, t4232, t4233, t4235)
}
