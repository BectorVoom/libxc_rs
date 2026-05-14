//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 906/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk906<F: Float>(t1341: F, t25313: F, t1340: F, t1339: F, t3512: F, t8093: F, t3759: F, t5602: F, t5606: F, t5600: F, t8089: F, t3739: F, t8079: F, t3764: F, t7906: F, t1415: F) -> (F, F, F, F, F, F, F) {
    let t25314 = t1341 * t25313;
    let t25315 = t1340 * t25314;
    let t25316 = t1339 * t25315;
    let t25318 = t3512 * t8093;
    let t25319 = t3759 * t25318;
    let t25321 = t5606 * t5602;
    let t25322 = t5600 * t25321;
    let t25324 = t3512 * t8089;
    let t25325 = t1339 * t25324;
    let t25327 = t3739 * t8079;
    let t25329 = t3764 * t7906;
    let t25330 = t1415 * t25329;
    (t25316, t25319, t25322, t25325, t25327, t25329, t25330)
}
