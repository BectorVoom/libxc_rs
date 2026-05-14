//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1044/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1044<F: Float>(t26816: F, t27055: F, t116: F, t7583: F, t2371: F, t25812: F, t25814: F, t25816: F, t25818: F, t25820: F, t25834: F, t26800: F, t26804: F, t670: F, t7586: F, t118: F, t1310: F, t1453: F, t2163: F, t2320: F, t2322: F, t2328: F, t2331: F, t25085: F, t25092: F, t25095: F, t25180: F, t25182: F, t25184: F, t25186: F, t25189: F, t508: F, t569: F, t649: F, t7584: F, t7591: F, t7683: F, t7687: F) -> (F, F, F, F) {
    let t27056 = t26816 + t27055;
    let t27060 = t7583 * t116;
    let t27066 = 2.0 * t2371 * t7586 + 4.0 * t27060 * t670 + t25812 + t25814 + t25816 + t25818 + t25820 + t25834 + t26800 + 2.0 * t26804;
    let t27075 = -t118 * t27056 - 2.0 * t1310 * t7584 + 2.0 * t1453 * t7687 - t2163 * t2320 - 2.0 * t2163 * t2328 - 4.0 * t2322 * t7591 - 4.0 * t2331 * t7586 - t26800 * t508 - 2.0 * t26804 * t508 + t27066 * t569 - 2.0 * t649 * t7683 - t25085 + t25092 - t25095 + t25180 - t25182 - t25184 - t25186 + t25189;
    (t27056, t27060, t27066, t27075)
}
