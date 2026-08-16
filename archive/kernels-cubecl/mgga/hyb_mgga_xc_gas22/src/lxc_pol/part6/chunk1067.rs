//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1067/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1067<F: Float>(t151: F, t4014: F, t3997: F, t716: F, t154: F, t10218: F, t10221: F, t10226: F, t10229: F, t10234: F, t10237: F, t10242: F, t10245: F, t3165: F, t3188: F, t3191: F, t3211: F, t3216: F, t3221: F, t708: F) -> (F, F, F, F) {
    let t10250 = t151 * t4014;
    let t10253 = t716 * t3997;
    let t10258 = t154 * t4014;
    let t10261 = t10218 * t708 / F::cast_from(258048.0_f64) + t10221 * t708 / F::cast_from(491520.0_f64) - t3211 * t3188 / F::cast_from(3440640.0_f64) - t10226 * t708 / F::cast_from(6881280.0_f64) - t10229 * t708 / F::cast_from(13271040.0_f64) + t3216 * t3188 / F::cast_from(0.10616832e9_f64) + t10234 * t708 / F::cast_from(0.21233664e9_f64) + t10237 * t708 / F::cast_from(412876800.0_f64) - t3221 * t3188 / F::cast_from(0.37158912e10_f64) - t10242 * t708 / F::cast_from(0.74317824e10_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t10245 * t708 + t3165 * t3188 / F::cast_from(3.0_f64) + t10250 * t708 / F::cast_from(6.0_f64) + t10253 * t708 / F::cast_from(8.0_f64) - t3191 * t3188 / F::cast_from(24.0_f64) - t10258 * t708 / F::cast_from(48.0_f64);
    (t10250, t10253, t10258, t10261)
}
