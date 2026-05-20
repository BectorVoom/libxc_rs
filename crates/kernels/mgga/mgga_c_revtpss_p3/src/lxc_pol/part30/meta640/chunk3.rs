//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2227/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2227<F: Float>(t104646: F, t17727: F, t17423: F, t29097: F, t17789: F, t29100: F, t17416: F, t7624: F, t17214: F, t17484: F, t17580: F, t17760: F, t29037: F, t29040: F, t29083: F, t3620: F, t3640: F, t3644: F, t97149: F, t97261: F) -> (F, F) {
    let t104647 = t17727 * t104646;
    let t104651 = F::cast_from(0.11433071498151929859e-2_f64) * t29097 * t17423;
    let t104653 = F::cast_from(0.57165357490759649296e-3_f64) * t29100 * t17789;
    let t104658 = t7624 * t17416;
    let t104666 = -F::cast_from(0.85748036236139473944e-3_f64) * t97149 * t17580 + F::cast_from(0.42874018118069736972e-3_f64) * t97261 * t17484 - F::cast_from(0.95275595817932748826e-3_f64) * t104647 * t17760 + t104651 - t104653 + F::cast_from(0.15244095330869239812e-2_f64) * t29083 * t3640 + F::cast_from(0.30488190661738479624e-2_f64) * t29083 * t3644 + F::cast_from(0.6351706387862183255e-4_f64) * t104658 + F::cast_from(0.47637797908966374413e-3_f64) * t29037 * t3620 - F::cast_from(0.2540682555144873302e-2_f64) * t29083 * t3620 - F::cast_from(0.57165357490759649296e-3_f64) * t29040 * t17214;
    (t104647, t104666)
}
