//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 880/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk880<F: Float>(t1286: F, t25912: F, t1307: F, t1308: F, t137376: F, t137398: F, t137400: F, t1526: F, t1527: F, t15567: F, t2258: F, t25905: F, t25909: F, t25916: F, t25956: F, t25969: F, t25972: F, t25975: F, t25978: F, t2984: F, t2993: F, t3000: F, t3052: F, t32033: F, t356: F, t5495: F, t5501: F, t5618: F, t5697: F, t6414: F, t6517: F, t8633: F, t925: F) -> (F,) {
    let t144472 = t1286 * t25912;
    let t144503 = -t5501 * t25975 / 9.0 + t5501 * t25978 / 27.0 - t5501 * t25969 / 9.0 - t5501 * t25972 / 9.0 + t5495 * t6517 / 3.0 + t1286 * t25916 / 3.0 - t144472 / 9.0 + t1286 * t25909 / 3.0 + t1286 * t25905 / 3.0 - t15567 * t8633 * t1307 * t2984 / 9.0 - t137376 / 54.0 + t137398 - t137400 / 12.0 + t6414 * t5697 / 3.0 + t15567 * t2258 * t1307 * t2993 / 6.0 + t6414 * t32033 / 18.0 + t1286 * t3000 * t1308 * t3052 / 9.0 + t1286 * t356 * t5618 * t925 / 18.0 - t1526 * t1527 * t25956 / 12.0;
    (t144503,)
}
