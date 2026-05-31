//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 584/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk584<F: Float>(t2596: F, t617: F, t1621: F, t1620: F, t1044: F, t1791: F, t661: F, t639: F, t1615: F, t1619: F, t1669: F, t2534: F, t2535: F, t2536: F, t2551: F, t2558: F, t2564: F, t2569: F, t2574: F, t2578: F, t2583: F, t2587: F, t2590: F, t2595: F, t267: F) -> (F, F, F, F, F, F, F, F) {
    let t2597 = t2596 * t617;
    let t2598 = t1621 * t2597;
    let t2600 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t1620 * t2598;
    let t2601 = t1791 * t1044;
    let t2602 = t2601 * t661;
    let t2603 = t1621 * t2602;
    let t2605 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t639 * t2603;
    let t2606 = -F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t1615 + t1619 + t2534 + t2535 - F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t2536 - t2551 * t267 / F::cast_from(15.0_f64) - t2558 + t2564 - t2569 + t2574 + t2578 + t2583 + t2587 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1669 + t2590 - t2595 - t2600 + t2605;
    (t2597, t2598, t2600, t2601, t2602, t2603, t2605, t2606)
}
