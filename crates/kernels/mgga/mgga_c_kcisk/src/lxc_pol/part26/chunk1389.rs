//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1389/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1389<F: Float>(t109518: F, t115661: F, t115663: F, t119336: F, t119339: F, t119351: F, t119354: F, t119357: F, t119364: F, t120046: F, t120149: F, t32439: F, t33767: F, t33823: F, t33827: F, t33941: F, t34955: F, t9536: F) -> (F,) {
    let t120536 = 0.31250000000000000001e-1 * t9536 * t120046 + 0.13402777777777777778e-2 * t109518 * t34955 + 0.13402777777777777778e-2 * t32439 * t120149 + t115661 + t115663 + 0.51588271604938271605e-2 * t119336 + 0.77382407407407407408e-2 * t119339 - 0.23214722222222222221e-2 * t119351 + 0.15476481481481481481e-2 * t119354 + 0.46429444444444444444e-2 * t119357 + 0.40208333333333333334e-2 * t33767 * t33823 - 0.30952962962962962962e-2 * t119364 - 0.69444444444444444444e-2 * t33941 * t33827;
    (t120536,)
}
