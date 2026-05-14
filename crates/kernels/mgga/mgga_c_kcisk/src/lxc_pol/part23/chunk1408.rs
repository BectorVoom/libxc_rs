//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1408/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1408<F: Float>(t32422: F, t9860: F, t20160: F, t33761: F, t32439: F, t33783: F, t33940: F, t9515: F, t20149: F, t33826: F, t9536: F, t2331: F, t33781: F, t4513: F, t6204: F, t1591: F, t32440: F, t6587: F) -> (F, F, F, F, F, F, F, F, F) {
    let t115073 = 0.34722222222222222222e-2 * t9860 * t32422;
    let t115075 = t20160 * t33761;
    let t115077 = 0.13402777777777777778e-2 * t32439 * t115075;
    let t115078 = t20160 * t33783;
    let t115080 = 0.40208333333333333334e-2 * t32439 * t115078;
    let t115085 = t9515 * t33940;
    let t115090 = t9536 * t20149 * t33826;
    let t115094 = t6204 * t33781 * t2331 * t4513;
    let t115099 = t6204 * t32440 * t6587 * t1591;
    (t115073, t115075, t115077, t115078, t115080, t115085, t115090, t115094, t115099)
}
