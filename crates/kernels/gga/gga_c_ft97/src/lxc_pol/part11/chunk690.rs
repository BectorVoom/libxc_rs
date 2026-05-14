//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 690/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk690<F: Float>(t10070: F, t2599: F, t2567: F, t2569: F, t684: F, t2606: F, t255: F, t9895: F, t2373: F, t258: F, t2492: F, t754: F, t2602: F, t10026: F, t10031: F, t10036: F, t10041: F, t10046: F, t10048: F, t10055: F, t10059: F, t10062: F, t10064: F, t10067: F, t1901: F, t446: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t10071 = t2599 * t10070;
    let t10074 = t2567 * t2569;
    let t10075 = t10074 * t684;
    let t10076 = t2606 * t10075;
    let t10079 = t9895 * t255;
    let t10080 = t258 * t2373;
    let t10081 = t10080 * t684;
    let t10082 = t10079 * t10081;
    let t10085 = t2492 * t754;
    let t10086 = t10085 * t2602;
    let t10089 = -10.0 / 81.0 * t446 * t10026 - 2.0 * t446 * t10031 - 2.0 * t446 * t10036 + 2.0 * t446 * t10041 + t446 * t10046 + t10048 / 3.0 - 2.0 * t446 * t10055 + 2.0 * t446 * t10059 - 2.0 / 3.0 * t10062 - 2.0 / 3.0 * t10064 - t446 * t10067 + t1901 * t10071 / 3.0 - 2.0 / 3.0 * t1901 * t10076 - 2.0 / 3.0 * t1901 * t10082 + 2.0 / 3.0 * t1901 * t10086;
    (t10071, t10074, t10075, t10076, t10079, t10080, t10081, t10082, t10085, t10086, t10089)
}
