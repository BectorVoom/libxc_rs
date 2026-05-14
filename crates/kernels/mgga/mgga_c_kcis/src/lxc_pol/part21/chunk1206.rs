//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1206/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1206<F: Float>(t26960: F, t97330: F, t1268: F, t9494: F, t26955: F, t13132: F, t13150: F, t28102: F, t28116: F, t5302: F, t5310: F, t7772: F, t92795: F, t93099: F, t93134: F, t96247: F, t96251: F, t96831: F, t97019: F) -> (F,) {
    let t97332 = 0.7722800925925925926e-4 * t26960 * t97330;
    let t97338 = t1268 * t9494;
    let t97344 = 0.10306077835648148148e-4 * t26955 * t97330;
    let t97347 = -0.30918233506944444444e-4 * t26955 * t97019 + 0.46377350260416666667e-4 * t7772 * t96831 + 0.61782407407407407408e-3 * t93099 - 0.61782407407407407408e-3 * t92795 * t28102 + t97332 + 0.23168402777777777778e-3 * t26960 * t5310 * t28116 * t13150 + 0.7722800925925925926e-4 * t93134 + 0.92673611111111111112e-3 * t26960 * t5302 * t97338 * t13132 + t97344 - 0.15476481481481481481e-2 * t96247 - 0.51588271604938271604e-3 * t96251;
    (t97347,)
}
