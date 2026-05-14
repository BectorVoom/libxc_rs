//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 871/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk871<F: Float>(t1882: F, t22412: F, t22226: F, t22364: F, t22255: F, t22380: F, t8392: F, t22210: F, t22398: F, t22461: F, t11902: F, t11906: F, t16052: F, t1871: F, t1901: F, t1902: F, t1909: F, t20182: F, t20191: F, t20291: F, t20434: F, t3238: F, t39167: F, t4454: F, t446: F, t4462: F, t452: F, t4572: F, t59629: F, t59684: F, t74759: F, t74786: F, t74809: F, t925: F, t986: F) -> (F, F, F, F, F, F, F, F, F) {
    let t84823 = t1882 * t22412;
    let t84825 = t1882 * t22226;
    let t84856 = t1882 * t22364;
    let t84880 = t1882 * t22255;
    let t84940 = t8392 * t22380;
    let t84958 = t1882 * t22210;
    let t84983 = t8392 * t22398;
    let t84985 = t8392 * t22461;
    let t85301 = -16.0 / 9.0 * t59629 - 8.0 / 9.0 * t74786 - 8.0 / 9.0 * t1901 * t39167 * t4572 * t4454 + 4.0 * t446 * t452 * t3238 * t20191 + 8.0 * t446 * t1871 * t986 * t20182 + 8.0 / 9.0 * t74809 + 16.0 / 9.0 * t59684 - 8.0 / 3.0 * t1901 * t11902 * t20291 + 4.0 / 3.0 * t1901 * t11906 * t20434 + 8.0 / 3.0 * t1901 * t1909 * t74759 * t925 + 2.0 / 3.0 * t1901 * t1902 * t16052 * t4462;
    (t84823, t84825, t84856, t84880, t84940, t84958, t84983, t84985, t85301)
}
