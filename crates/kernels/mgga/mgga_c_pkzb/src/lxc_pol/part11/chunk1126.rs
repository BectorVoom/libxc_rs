//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1126/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1126<F: Float>(t21184: F, t9225: F, t17351: F, t17405: F, t17454: F, t20705: F, t25633: F, t25636: F, t25734: F, t25740: F, t25747: F, t25750: F, t25767: F, t30284: F, t30287: F, t30289: F, t30291: F, t30294: F, t30296: F, t30309: F, t30311: F) -> (F, F) {
    let t30502 = 18.0 * t21184 * t9225;
    let t30525 = -0.73586666666666666666e0 * t17405 - 0.28179666666666666667e1 * t20705 + 0.258925e1 * t30289 + 0.16504875e0 * t30291 + 0.58258125e1 * t30294 - 0.1237865625e0 * t30296 + t17454 - 0.93932222222222222223e0 * t17351 + 0.12077e1 * t25633 - 0.905775e0 * t25636 + 0.82785e0 * t25734 - 0.301925e0 * t30284 + 0.905775e0 * t30287 - 0.99342e0 * t25740 - 0.49671e0 * t25747 - 0.49671e0 * t25750 + 0.82785e0 * t25767 + 0.6189328125e-1 * t30309 - 0.1237865625e0 * t30311;
    (t30502, t30525)
}
