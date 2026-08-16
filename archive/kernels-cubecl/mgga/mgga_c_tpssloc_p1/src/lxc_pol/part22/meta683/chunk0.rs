//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2249/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2249<F: Float>(t17620: F, t2960: F, t5893: F, t698: F, t973: F, t17615: F, t3131: F, t5866: F, t1022: F, t5872: F, t10403: F, t10422: F, t18015: F) -> (F, F, F, F, F, F, F) {
    let t62827 = t2960 * t17620;
    let t62832 = t973 * t698 * t5893;
    let t62836 = t2960 * t17615;
    let t62840 = t5866 * t3131;
    let t62845 = t5866 * t1022;
    let t62850 = t5872 * t1022;
    let t62891 = t10403 * t10422 * t18015;
    (t62827, t62832, t62836, t62840, t62845, t62850, t62891)
}
