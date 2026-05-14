//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1147/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1147<F: Float>(t11108: F, t1306: F, t17753: F, t30273: F, t30275: F, t30277: F, t30362: F, t30364: F, t30366: F, t30369: F, t30379: F, t30381: F, t30385: F, t803: F, t2993: F, t30387: F, t30502: F, t30704: F, t30706: F, t30708: F, t30710: F, t30714: F, t30716: F, t30718: F, t30722: F, t9721: F) -> (F, F) {
    let t30998 = -6.0 * t11108 * t1306 * t17753 * t803 + t30273 - t30275 - t30277 + t30362 + t30364 + t30366 + t30369 + t30379 + t30381 + t30385;
    let t31004 = 6.0 * t1306 * t2993 * t9721 + t30387 + t30502 + t30704 - t30706 + t30708 + t30710 + t30714 - t30716 + t30718 + t30722;
    (t30998, t31004)
}
