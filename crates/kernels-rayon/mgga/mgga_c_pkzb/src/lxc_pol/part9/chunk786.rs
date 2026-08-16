//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 786/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk786(t1843: f64, t655: f64, t218: f64, t219: f64, t208: f64, t5537: f64, t5513: f64, t5516: f64, t5522: f64, t5525: f64, t5539: f64, t5541: f64, t5543: f64, t5548: f64, t5551: f64, t5553: f64, t5558: f64, t5560: f64, t5563: f64, t5566: f64) -> (f64, f64, f64, f64, f64) {
    let t5568 = t655 * t1843;
    let t5570 = t218 * t219 * t5568;
    let t5572 = t208 * t5537;
    let t5574 = t218 * t219 * t5572;
    let t5576 = 0.19419375e1_f64 * t5513 - 0.3883875e1_f64 * t5516 + 0.258925e1_f64 * t5541 - t5543 + 0.12077e1_f64 * t5522 - 0.905775e0_f64 * t5525 + 0.905775e0_f64 * t5539 - 0.412621875e-1_f64 * t5548 + 0.247573125e0_f64 * t5551 + 0.16504875e0_f64 * t5553 - t5558 + 0.82785e0_f64 * t5560 - 0.49671e0_f64 * t5563 - 0.49671e0_f64 * t5566 + 0.745065e0_f64 * t5570 + 0.248355e0_f64 * t5574;
    (t5568, t5570, t5572, t5574, t5576)
}
