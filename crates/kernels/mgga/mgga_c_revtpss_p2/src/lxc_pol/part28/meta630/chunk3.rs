//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2275/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2275<F: Float>(t1455: F, t7956: F, t1464: F, t7939: F, t2037: F, t5808: F, t1921: F, t7318: F, t2045: F, t5789: F, t18178: F, t18217: F, t2038: F, t28235: F, t4154: F, t5790: F, t7337: F, t92556: F, t95125: F, t95180: F) -> F {
    let t101661 = F::cast_from(2.0_f64) * t1455 * t7956;
    let t101668 = F::cast_from(2.0_f64) * t7939 * t1464;
    let t101670 = F::cast_from(2.0_f64) * t2037 * t5808;
    let t101672 = F::cast_from(2.0_f64) * t7318 * t1921;
    let t101674 = F::cast_from(2.0_f64) * t5789 * t2045;
    let t101678 = F::cast_from(2.0_f64) * t1464 * t28235 + t18178 * t2045 + t18217 * t2038 + t4154 * t7956 + F::cast_from(2.0_f64) * t5790 * t7337 + t101661 + t101668 + t101670 + t101672 + t101674 + t92556 + F::cast_from(2.0_f64) * t95125 + t95180;
    t101678
}
