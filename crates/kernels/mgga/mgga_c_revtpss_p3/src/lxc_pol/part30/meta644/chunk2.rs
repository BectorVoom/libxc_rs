//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2263/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2263<F: Float>(t101439: F, t101472: F, t101476: F, t101482: F, t101485: F, t101486: F, t101546: F, t101548: F, t101550: F, t101552: F, t13521: F, t13532: F, t14310: F, t1843: F, t2165: F, t26800: F, t26804: F, t3813: F, t4151: F, t5517: F, t5787: F, t7584: F, t7586: F, t7687: F, t8152: F, t8237: F) -> F {
    let t105756 = -F::cast_from(2.0_f64) * t13521 * t7586 - F::cast_from(4.0_f64) * t13532 * t7586 + t14310 * t2165 - t1843 * t26800 - F::cast_from(2.0_f64) * t1843 * t26804 - t3813 * t8152 + t4151 * t8237 - F::cast_from(2.0_f64) * t5517 * t7584 + F::cast_from(2.0_f64) * t5787 * t7687 + t101439 - t101472 + t101476 - t101482 - t101485 - t101486 + t101546 - t101548 - t101550 - t101552;
    t105756
}
