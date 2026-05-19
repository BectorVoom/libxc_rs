//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 389/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk389<F: Float>(t1801: F, t2527: F, t1873: F, t1869: F, t2454: F, t719: F, t717: F, t415: F, t1899: F, t2441: F, t1800: F, t1693: F, t1796: F, t2399: F, t2470: F, t2475: F, t2511: F, t671: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t2528 = t1801 * t2527;
    let t2529 = t1873 * t2528;
    let t2530 = t1869 * t2529;
    let t2532 = sigma2 * t2454;
    let t2533 = t2532 * t719;
    let t2534 = t717 * t2533;
    let t2535 = t415 * t2534;
    let t2537 = t1899 * t2441;
    let t2538 = t1800 * t2537;
    let t2539 = t1869 * t2538;
    let t2541 = t2399 * t671 - F::new(0.193e0) * t1693 * t2470 + t1796 + F::cast_from(0.16581944444444444444e-2_f64) * t2475 + F::cast_from(0.24872916666666666666e-2_f64) * t2511 - F::cast_from(0.24872916666666666666e-2_f64) * t2530 - F::cast_from(0.66327777777777777776e-2_f64) * t2535 + F::cast_from(0.16581944444444444444e-2_f64) * t2539;
    (t2528, t2529, t2530, t2532, t2533, t2534, t2535, t2537, t2538, t2539, t2541)
}
