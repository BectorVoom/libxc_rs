//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1708/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1708<F: Float>(t1011: F, t1012: F, t11859: F, t3117: F, t3155: F, t42508: F, t6271: F, t6299: F, t67015: F, t67186: F, t67195: F, t67206: F, t79811: F, t79818: F, t79874: F, t79881: F, t79892: F, t79938: F, t79944: F, t79946: F, t87145: F) -> F {
    let t89306 = t79811 / F::cast_from(54.0_f64) + F::cast_from(0.17149607247227894789e-2_f64) * t79818 + F::cast_from(0.57165357490759649296e-3_f64) * t67015 - F::cast_from(7.0_f64) / F::cast_from(54.0_f64) * t1011 * t1012 * t42508 * t87145 - F::cast_from(0.17149607247227894789e-2_f64) * t79874 - t79881 / F::cast_from(27.0_f64) + F::cast_from(0.28582678745379824648e-3_f64) * t67186 + F::cast_from(0.57165357490759649296e-3_f64) * t67195 + F::cast_from(0.34299214494455789578e-2_f64) * t79892 - F::cast_from(0.57165357490759649296e-3_f64) * t67206 - F::cast_from(0.51448821741683684368e-2_f64) * t11859 * t3117 * t6271 * t3155 * t6299 + F::cast_from(0.22866142996303859719e-2_f64) * t79938 - t79944 / F::cast_from(36.0_f64) - F::cast_from(0.17149607247227894789e-2_f64) * t79946;
    t89306
}
