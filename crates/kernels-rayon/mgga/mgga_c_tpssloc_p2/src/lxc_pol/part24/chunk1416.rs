//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1416/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1416(t12303: f64, t24995: f64, t8945: f64, t113: f64, t12504: f64, t1976: f64, t22483: f64, t22619: f64, t2314: f64, t2363: f64, t4034: f64, t6517: f64, t652: f64, t6539: f64, t6862: f64, t81426: f64, t81430: f64, t81432: f64, t81434: f64, t81458: f64, t81469: f64, t83554: f64, t83666: f64, t83672: f64, t83674: f64, t83677: f64, t83679: f64, t83681: f64, t83684: f64, t9348: f64, t9416: f64) -> f64 {
    let t83687 = 18.0_f64 * t24995 * t8945 * t12303;
    let t83688 = t81426 - 6.0_f64 * t6517 * t12504 - t81430 - t81432 - t81434 - 6.0_f64 * t9348 * t6539 - t81458 - 6.0_f64 * t4034 * t22483 - 6.0_f64 * t652 * t6862 * t2363 - 2.0_f64 * t652 * t1976 * t9416 + t81469 - t113 * (t83554 + t83666) - 12.0_f64 * t2314 * t22619 - t83672 - t83674 - t83677 - t83679 - t83681 - t83684 + t83687;
    t83688
}
