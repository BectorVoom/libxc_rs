//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 460/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk460(t668: f64, t761: f64, t505: f64, t766: f64, t2606: f64, t2409: f64, t265: f64, t724: f64, t1901: f64, t193: f64, t2471: f64, t2528: f64, t2544: f64, t2549: f64, t2553: f64, t2554: f64, t2556: f64, t2559: f64, t2563: f64, t2571: f64, t2576: f64, t2581: f64, t2584: f64, t2587: f64, t2591: f64, t2596: f64, t2603: f64, t446: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2607 = t761 * t668;
    let t2608 = t505 * t766;
    let t2609 = t2607 * t2608;
    let t2610 = t2606 * t2609;
    let t2614 = t724 * t265 * t2409;
    let t2617 = -2.0_f64 / 3.0_f64 * t446 * t2471 - t446 * t2528 / 3.0_f64 + t89 * t193 * t2544 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t2549 + t2553 + 2.0_f64 / 9.0_f64 * t2554 + 2.0_f64 / 9.0_f64 * t2556 - 2.0_f64 / 3.0_f64 * t446 * t2559 - t446 * t2563 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t2571 + 2.0_f64 / 3.0_f64 * t446 * t2576 + 2.0_f64 / 3.0_f64 * t446 * t2581 + 2.0_f64 / 27.0_f64 * t2584 - 2.0_f64 / 9.0_f64 * t446 * t2587 - t446 * t2591 / 9.0_f64 - 2.0_f64 / 27.0_f64 * t446 * t2596 + 2.0_f64 / 9.0_f64 * t1901 * t2603 + 2.0_f64 / 9.0_f64 * t1901 * t2610 + 2.0_f64 / 9.0_f64 * t446 * t2614;
    (t2607, t2608, t2609, t2610, t2614, t2617)
}
