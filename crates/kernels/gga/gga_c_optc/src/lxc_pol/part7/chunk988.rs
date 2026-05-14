//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 988/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk988<F: Float>(t714: F, t7142: F, t108: F, t176: F, t203: F, t616: F, t6599: F, t1948: F, t2226: F, t6560: F, t729: F, t1972: F, t1974: F, t1956: F, t2229: F, t104: F, t1879: F, t22340: F, t22342: F, t22344: F, t22621: F, t22623: F, t22625: F, t22627: F, t6312: F, t6856: F, t712: F, t95: F) -> (F, F, F, F, F) {
    let t23360 = t7142 * t714;
    let t23373 = t176 * t6599 * t616 * t108 * t203;
    let t23378 = t176 * t2226 * t1948 * t108 * t203;
    let t23383 = t176 * t729 * t6560 * t108 * t203;
    let t23390 = t1972 * t1972;
    let t23392 = t1974 * t1974;
    let t23393 = 1.0 / t23392;
    let t23400 = t2229 * t1956;
    let t23402 = t22340 + 0.62027715443768233192e-1 * t95 * t6856 * t712 * t714 + t22342 - 0.15506928860942058298e-1 * t95 * t104 * t23390 * t23393 + t22344 + t22621 - t22623 + t22625 - t22627 + 0.46520786582826174894e-1 * t1879 * t6312 * t1948 + 3.0 * t23400;
    (t23360, t23373, t23378, t23383, t23402)
}
