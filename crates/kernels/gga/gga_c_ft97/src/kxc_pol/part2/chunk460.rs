//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 460/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk460<F: Float>(t668: F, t761: F, t505: F, t766: F, t2606: F, t2409: F, t265: F, t724: F, t1901: F, t193: F, t2471: F, t2528: F, t2544: F, t2549: F, t2553: F, t2554: F, t2556: F, t2559: F, t2563: F, t2571: F, t2576: F, t2581: F, t2584: F, t2587: F, t2591: F, t2596: F, t2603: F, t446: F, t89: F) -> (F, F, F, F, F, F) {
    let t2607 = t761 * t668;
    let t2608 = t505 * t766;
    let t2609 = t2607 * t2608;
    let t2610 = t2606 * t2609;
    let t2614 = t724 * t265 * t2409;
    let t2617 = -F::new(2.0) / F::new(3.0) * t446 * t2471 - t446 * t2528 / F::new(3.0) + t89 * t193 * t2544 / F::new(3.0) - F::new(2.0) / F::new(9.0) * t2549 + t2553 + F::new(2.0) / F::new(9.0) * t2554 + F::new(2.0) / F::new(9.0) * t2556 - F::new(2.0) / F::new(3.0) * t446 * t2559 - t446 * t2563 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t446 * t2571 + F::new(2.0) / F::new(3.0) * t446 * t2576 + F::new(2.0) / F::new(3.0) * t446 * t2581 + F::new(2.0) / F::new(27.0) * t2584 - F::new(2.0) / F::new(9.0) * t446 * t2587 - t446 * t2591 / F::new(9.0) - F::new(2.0) / F::new(27.0) * t446 * t2596 + F::new(2.0) / F::new(9.0) * t1901 * t2603 + F::new(2.0) / F::new(9.0) * t1901 * t2610 + F::new(2.0) / F::new(9.0) * t446 * t2614;
    (t2607, t2608, t2609, t2610, t2614, t2617)
}
