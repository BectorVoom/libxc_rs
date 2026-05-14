//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1436/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1436<F: Float>(t1596: F, t32440: F, t6204: F, t6581: F, t33766: F, t9528: F, t114715: F, t110064: F, t110066: F, t110068: F, t114684: F, t114687: F, t114694: F, t114700: F, t114704: F, t114707: F, t114718: F, t32439: F, t9519: F, t9536: F) -> (F,) {
    let t115866 = t6204 * t32440 * t6581 * t1596;
    let t115871 = t33766 * t9528;
    let t115883 = 0.23214722222222222222e-2 * t114715;
    let t115885 = -0.10416666666666666667e-1 * t9536 * t115866 - 0.40208333333333333334e-2 * t32439 * t115866 - 0.10722222222222222222e-1 * t115871 * t9519 + 0.92858888888888888886e-2 * t114684 - 0.23214722222222222222e-2 * t114687 + 0.77382407407407407407e-3 * t110064 + 0.12897067901234567901e-2 * t110066 + 0.77382407407407407406e-3 * t110068 + 0.11349419753086419753e-1 * t114694 - 0.17024129629629629629e-1 * t114700 + 0.34822083333333333332e-2 * t114704 + 0.92858888888888888886e-2 * t114707 - t115883 - 0.61905925925925925926e-2 * t114718;
    (t115885,)
}
