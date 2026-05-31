//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1285/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1285<F: Float>(t23699: F, t7866: F, t9839: F, t10081: F, t1181: F, t1861: F, t19: F, t23043: F, t23647: F, t23649: F, t23655: F, t23746: F, t23749: F, t23756: F, t23759: F, t23762: F, t23765: F, t26: F, t27038: F, t27649: F, t27732: F, t2949: F, t2950: F, t2970: F, t3: F, t3114: F, t3917: F, t547: F, t668: F, t7835: F, t7842: F, t7852: F, t7856: F, t7868: F, t8148: F, t8201: F, t9846: F, t9851: F) -> F {
    let t27815 = t7866 * t23699 * t9839;
    let t27843 = -F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t7866 * t7868 * t27732 - F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t23647 * t23649 * t27649 - t2970 * t23043 * t9846 / F::cast_from(6.0_f64) - t2970 * t7835 * t7856 * t3 / F::cast_from(6.0_f64) + t7842 * t7852 * t9839 / F::cast_from(8.0_f64) - F::cast_from(7.0_f64) / F::cast_from(216.0_f64) * t27815 - t23746 / F::cast_from(32.0_f64) - t23749 / F::cast_from(16.0_f64) - F::cast_from(3.0_f64) / F::cast_from(32.0_f64) * t547 * t9851 - F::cast_from(3.0_f64) / F::cast_from(32.0_f64) * t19 * t26 * t10081 * t668 - F::cast_from(3.0_f64) / F::cast_from(64.0_f64) * t19 * t26 * t3917 * t1861 - F::cast_from(3.0_f64) / F::cast_from(32.0_f64) * t1181 * t8148 - F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t1181 * t8201 - F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t2949 * t2950 * t3114 - F::cast_from(7.0_f64) / F::cast_from(18.0_f64) * t7866 * t23655 * t27038 + t23756 / F::cast_from(72.0_f64) - t23759 / F::cast_from(48.0_f64) - t23762 / F::cast_from(48.0_f64) - t23765 / F::cast_from(96.0_f64);
    t27843
}
