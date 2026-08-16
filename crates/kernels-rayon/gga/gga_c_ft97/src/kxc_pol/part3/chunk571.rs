//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 571/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk571(t86: f64, t112: f64, t113: f64, t4628: f64, t4635: f64, t5: f64, t989: f64, t992: f64, t1943: f64, t920: f64, t1017: f64, t72: f64, t1023: f64, t1526: f64, t1527: f64, t1942: f64, t342: f64, t343: f64) -> (f64, f64, f64, f64) {
    let t87 = 10000000.0_f64 <= t86;
    let t4640 = piecewise3(t87, 0.0_f64, t5 * t4628 * t113 / 4.0_f64 + t5 * t989 * t992 / 2.0_f64 + t5 * t112 * t4635 / 4.0_f64);
    let t4641 = t1943 * t920;
    let t4645 = t72 * t1017;
    let t4649 = t1023 - t1942 - t1526 * t1527 * t4641 / 12.0_f64 - t342 * t343 * t4645 / 4.0_f64;
    (t4640, t4641, t4645, t4649)
}
