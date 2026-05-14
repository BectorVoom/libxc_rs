//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1200/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1200<F: Float>(t23699: F, t7866: F, t9839: F, t10081: F, t1181: F, t1861: F, t19: F, t23043: F, t23647: F, t23649: F, t23655: F, t23746: F, t23749: F, t23756: F, t23759: F, t23762: F, t23765: F, t26: F, t27038: F, t27649: F, t27732: F, t2949: F, t2950: F, t2970: F, t3: F, t3114: F, t3917: F, t547: F, t668: F, t7835: F, t7842: F, t7852: F, t7856: F, t7868: F, t8148: F, t8201: F, t9846: F, t9851: F) -> (F,) {
    let t27815 = t7866 * t23699 * t9839;
    let t27843 = -7.0 / 144.0 * t7866 * t7868 * t27732 - 35.0 / 216.0 * t23647 * t23649 * t27649 - t2970 * t23043 * t9846 / 6.0 - t2970 * t7835 * t7856 * t3 / 6.0 + t7842 * t7852 * t9839 / 8.0 - 7.0 / 216.0 * t27815 - t23746 / 32.0 - t23749 / 16.0 - 3.0 / 32.0 * t547 * t9851 - 3.0 / 32.0 * t19 * t26 * t10081 * t668 - 3.0 / 64.0 * t19 * t26 * t3917 * t1861 - 3.0 / 32.0 * t1181 * t8148 - 3.0 / 16.0 * t1181 * t8201 - 3.0 / 8.0 * t2949 * t2950 * t3114 - 7.0 / 18.0 * t7866 * t23655 * t27038 + t23756 / 72.0 - t23759 / 48.0 - t23762 / 48.0 - t23765 / 96.0;
    (t27843,)
}
