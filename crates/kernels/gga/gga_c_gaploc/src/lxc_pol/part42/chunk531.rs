//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 531/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk531<F: Float>(t1016: F, t2497: F, t3381: F, t4379: F, t2366: F, t2754: F, t2365: F, t1429: F, t10241: F, t447: F, t6964: F, t6963: F) -> (F, F, F, F, F, F, F) {
    let t10301 = t1016 * t2497;
    let t10308 = t4379 * t3381;
    let t10309 = F::new(0.14896037479937677779e-1) * t10308;
    let t10310 = t2366 * t2754;
    let t10311 = t2365 * t10310;
    let t10312 = t1429 * t10311;
    let t10313 = F::new(0.14896037479937677779e-1) * t10312;
    let t10314 = t10241 * t447;
    let t10315 = t6964 * t10314;
    let t10317 = F::new(0.71500979903700853338e0) * t6963 * t10315;
    (t10301, t10308, t10309, t10312, t10313, t10314, t10317)
}
