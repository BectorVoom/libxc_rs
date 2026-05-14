//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1221/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1221<F: Float>(t100055: F, t116250: F, t116807: F, t116848: F, t116893: F, t116937: F, t116980: F, t117017: F, t117041: F, t117276: F, t117316: F, t117336: F, t117355: F, t117387: F, t117435: F, t117476: F, t117520: F, t117564: F, t117597: F, t117649: F, t117699: F, t117746: F, t117788: F, t117841: F, t117886: F, t117918: F, t117952: F, t117992: F, t118041: F, t118085: F, t118108: F, t118130: F, t118168: F, t118207: F, t118253: F, t118303: F, t118339: F, t1337: F, t15594: F, t1642: F, t22907: F, t25611: F, t25617: F, t2976: F, t30018: F, t3204: F, t378: F, t438: F, t4501: F, t5501: F, t5748: F, t6455: F, t6562: F, t88: F, t94024: F, t94036: F) -> (F,) {
    let t118367 = t116250 / 27.0 - 2.0 * t116807 - t88 * (t116980 + t117649 + t117476 + t118085 + t117952 + t118130 + t117699 + t117597 + t118253 + t117992 + t117017 + t117564 + t118168 + t118339 + t117355 + t118207 + t117316 + t118303 + t117276 + t116893 + t117041 + t116937 + t116848 + t117841 + t117886 + t118041 + t117918 + t117520 + t117387 + t117788 + t117746 + t117435) + 2.0 / 9.0 * t5501 * t22907 * t100055 * t3204 + 2.0 / 9.0 * t5501 * t378 * t6455 * t25611 - 2.0 / 27.0 * t5501 * t1642 * t6455 * t25617 - t4501 * t5748 - t15594 * t1337 - t438 * t30018 + 2.0 / 27.0 * t94024 - 2.0 / 81.0 * t94036 - 2.0 * t2976 * t6562 + 8.0 * t118108 + 8.0 * t117336;
    (t118367,)
}
