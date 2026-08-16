//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1159/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1159<F: Float>(t12345: F, t3876: F, t22843: F, t241: F, t67: F, t3872: F, t12353: F, t3866: F, t12339: F, t12211: F, t12375: F, t12012: F, t12215: F, t12240: F, t12305: F, t12336: F, t12368: F, t1328: F, t1363: F, t210: F, t3719: F, t3733: F, t3765: F, t3783: F, t3870: F, t39622: F, t40026: F, t5246: F, t5248: F, t820: F) -> F {
    let t40065 = t12345 * t3876;
    let t40070 = t241 * t22843 * t67;
    let t40079 = t12345 * t3872;
    let t40081 = t3866 * t12353;
    let t40083 = t12339 * t3872;
    let t40089 = t12211 * t12375;
    let t40101 = F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t12336 * t3872 - F::cast_from(119.0_f64) / F::cast_from(576.0_f64) * t40065 - F::cast_from(5.0_f64) / F::cast_from(32.0_f64) * t3783 * t12353 + F::cast_from(35.0_f64) / F::cast_from(128.0_f64) * t1363 * t40070 * t820 * t40026 + F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t1363 * t3870 * t820 * t39622 + F::cast_from(595.0_f64) / F::cast_from(576.0_f64) * t40079 + F::cast_from(35.0_f64) / F::cast_from(48.0_f64) * t40081 - F::cast_from(35.0_f64) / F::cast_from(96.0_f64) * t40083 + F::cast_from(3.0_f64) / F::cast_from(256.0_f64) * t5246 * t5248 * t12368 * t12240 - F::cast_from(7.0_f64) / F::cast_from(4.0_f64) * t40089 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t12215 * t210 * t3765 * t3719 + t3733 * t210 * t1328 * t12012 / F::cast_from(4.0_f64) + F::cast_from(5.0_f64) / F::cast_from(64.0_f64) * t3783 * t12305;
    t40101
}
