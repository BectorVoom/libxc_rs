//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2312/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2312(t1920: f64, t28630: f64, t968: f64, t5872: f64, t6768: f64, t83244: f64, t89242: f64, t1058: f64, t1060: f64, t18047: f64, t1948: f64, t23346: f64, t28663: f64, t3200: f64, t3201: f64, t345: f64, t4649: f64, t5838: f64, t6687: f64, t6805: f64, t7593: f64, t89360: f64, t89362: f64, t89366: f64, t89369: f64, t89505: f64, t986: f64) -> (f64, f64, f64) {
    let t100324 = t1920 * t968 * t28630;
    let t100326 = t6768 * t5872;
    let t100334 = t83244 * t89242;
    let t100341 = -t89360 - t89362 - 0.12184696791468343974e-2_f64 * t89366 + t89369 + 0.82246703342411321825e-2_f64 * t1920 * t345 * t1948 * t18047 - 0.82246703342411321825e-2_f64 * t6687 * t5838 * t6805 + 0.27415567780803773942e-2_f64 * t100324 - t3200 * t100326 * t3201 - 0.82246703342411321825e-2_f64 * t6687 * t986 * t28630 + 0.43864908449286038307e-1_f64 * t23346 * t28663 + 0.16449340668482264365e-1_f64 * t100334 * t89505 + 2.0_f64 * t1058 * t7593 * t4649 * t1060;
    (t100326, t100334, t100341)
}
