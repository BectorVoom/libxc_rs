//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 991/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk991(t22246: f64, t8392: f64, t1882: f64, t22412: f64, t22226: f64, t22364: f64, t22255: f64, t22380: f64, t22210: f64, t22398: f64, t22461: f64, t11902: f64, t11906: f64, t16052: f64, t1871: f64, t1901: f64, t1902: f64, t1909: f64, t20182: f64, t20191: f64, t20291: f64, t20434: f64, t3238: f64, t39167: f64, t4454: f64, t446: f64, t4462: f64, t452: f64, t4572: f64, t59629: f64, t59684: f64, t74759: f64, t74786: f64, t74809: f64, t925: f64, t986: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t84797 = t8392 * t22246;
    let t84823 = t1882 * t22412;
    let t84825 = t1882 * t22226;
    let t84856 = t1882 * t22364;
    let t84880 = t1882 * t22255;
    let t84940 = t8392 * t22380;
    let t84958 = t1882 * t22210;
    let t84983 = t8392 * t22398;
    let t84985 = t8392 * t22461;
    let t85301 = -16.0_f64 / 9.0_f64 * t59629 - 8.0_f64 / 9.0_f64 * t74786 - 8.0_f64 / 9.0_f64 * t1901 * t39167 * t4572 * t4454 + 4.0_f64 * t446 * t452 * t3238 * t20191 + 8.0_f64 * t446 * t1871 * t986 * t20182 + 8.0_f64 / 9.0_f64 * t74809 + 16.0_f64 / 9.0_f64 * t59684 - 8.0_f64 / 3.0_f64 * t1901 * t11902 * t20291 + 4.0_f64 / 3.0_f64 * t1901 * t11906 * t20434 + 8.0_f64 / 3.0_f64 * t1901 * t1909 * t74759 * t925 + 2.0_f64 / 3.0_f64 * t1901 * t1902 * t16052 * t4462;
    (t84797, t84823, t84825, t84856, t84880, t84940, t84958, t84983, t84985, t85301)
}
