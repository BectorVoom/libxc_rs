//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1293/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1293<F: Float>(t1238: F, t2024: F, t2027: F, t23804: F, t23975: F, t23977: F, t23985: F, t23987: F, t23990: F, t23992: F, t23994: F, t23996: F, t23999: F, t24455: F, t28046: F, t28049: F, t28057: F, t28060: F, t28066: F, t3: F, t3150: F, t3925: F, t6457: F, t684: F, t687: F, t8492: F) -> F {
    let t28084 = -t28046 / F::cast_from(32.0_f64) - t28049 / F::cast_from(32.0_f64) - F::cast_from(5.0_f64) / F::cast_from(144.0_f64) * t23975 + t23977 / F::cast_from(24.0_f64) + t23985 / F::cast_from(48.0_f64) + t23987 / F::cast_from(24.0_f64) - F::cast_from(5.0_f64) / F::cast_from(144.0_f64) * t23990 - t23992 / F::cast_from(32.0_f64) + t28057 / F::cast_from(96.0_f64) + t28060 / F::cast_from(96.0_f64) - t23994 / F::cast_from(32.0_f64) - t23996 / F::cast_from(16.0_f64) + t23999 / F::cast_from(24.0_f64) + t28066 / F::cast_from(216.0_f64) - t2024 * t2027 * t6457 * t3925 / F::cast_from(48.0_f64) - t684 * t687 * t23804 * t1238 / F::cast_from(32.0_f64) + t684 * t3150 * t8492 * t3 / F::cast_from(8.0_f64) - t684 * t687 * t24455 * t1238 / F::cast_from(32.0_f64);
    t28084
}
