//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2243/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2243<F: Float>(t104624: F, t104626: F, t104636: F, t104640: F, t104666: F, t104692: F, t104718: F, t104746: F, t104772: F, t104796: F, t104821: F, t104825: F, t104828: F, t104833: F, t104834: F, t104844: F, t104876: F, t104900: F, t104921: F, t104951: F, t104973: F, t104992: F, t105017: F, t1252: F, t12966: F, t17222: F, t17237: F, t17254: F, t17700: F, t17796: F, t1808: F, t26852: F, t26880: F, t29020: F, t29040: F, t3591: F, t3714: F, t5386: F, t5397: F, t7618: F, t7623: F, t7624: F, t97112: F, t97138: F, t97200: F) -> F {
    let t105021 = t104746 + t104844 + t104921 + t104900 + t104992 - F::cast_from(0.19055119163586549765e-3_f64) * t97200 + t104833 + t104951 + t104718 + t104796 + t104828 + t104876 - t104624 + t104626 - t104640 + t105017 + F::cast_from(0.28582678745379824648e-3_f64) * t97112 + t104821 + F::cast_from(0.17149607247227894789e-2_f64) * t12966 * t7623 * t5386 + t104973 + F::cast_from(0.95275595817932748827e-4_f64) * t104825 + t104772 + t104692 + t104666 - F::cast_from(0.22866142996303859718e-2_f64) * t29020 * t3591 - F::cast_from(0.45732285992607719436e-2_f64) * t104834 * t1252 + F::cast_from(0.95275595817932748826e-3_f64) * t7624 * t17700 - F::cast_from(0.47637797908966374413e-3_f64) * t26880 * t17796 + F::cast_from(0.17149607247227894789e-2_f64) * t29040 * t17254 - F::cast_from(0.28582678745379824648e-3_f64) * t97138 * t1808 - F::cast_from(0.57165357490759649296e-3_f64) * t26852 * t5397 - F::cast_from(0.1270341277572436651e-2_f64) * t7624 * t17237 - F::cast_from(0.30488190661738479624e-2_f64) * t104636 * t3714 + F::cast_from(0.42874018118069736972e-3_f64) * t7618 * t17222;
    t105021
}
