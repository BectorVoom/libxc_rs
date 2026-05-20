//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2801/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2801<F: Float>(t22590: F, t625: F, t22593: F, t1513: F, t5915: F, t22629: F, t1504: F, t5823: F, t22: F, t39454: F, t100: F, t13475: F, t2: F, t21850: F, t2255: F, t22596: F, t22597: F, t22600: F, t22604: F, t22605: F, t22608: F, t2349: F, t4269: F, t4280: F, t46196: F, t49777: F, t580: F, t5895: F, t5902: F, t656: F, t658: F, t662: F, t97: F) -> (F, F, F, F, F, F) {
    let t75822 = t625 * t22590;
    let t75831 = t625 * t22593;
    let t75833 = t1513 * t5915;
    let t75843 = t625 * t22629;
    let t75861 = t1504 * t5823;
    let t75879 = F::new(6.0) * t22 + F::new(12.0) * t39454;
    let t75887 = F::new(50.0) / F::new(81.0) * t656 * t22597 + F::new(40.0) / F::new(81.0) * t97 * t46196 * t22596 * t658 - F::new(10.0) / F::new(9.0) * t49777 * t5895 * t2 * t580 - F::new(50.0) / F::new(9.0) * t656 * t22600 - F::new(10.0) / F::new(9.0) * t49777 * t75861 * t658 + F::new(10.0) / F::new(3.0) * t13475 * t2255 * t5823 + F::new(10.0) / F::new(3.0) * t97 * t4269 * t21850 - F::new(25.0) / F::new(9.0) * t656 * t22605 + F::new(10.0) / F::new(9.0) * t97 * t2349 * t22604 * t658 + F::new(5.0) / F::new(3.0) * t97 * t100 * t75879 - F::new(2200.0) / F::new(81.0) * t22608 * t662 + F::new(400.0) / F::new(27.0) * t5902 * t4280;
    (t75822, t75831, t75833, t75843, t75879, t75887)
}
