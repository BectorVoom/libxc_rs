//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1304/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1304(t20816: f64, t4205: f64, t67230: f64, t67243: f64, t58972: f64, t67463: f64, t17116: f64, t1877: f64, t2522: f64, t28248: f64, t39549: f64, t39563: f64, t39585: f64, t39590: f64, t40799: f64, t40801: f64, t40803: f64, t5664: f64, t59564: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t75939 = 16.0_f64 * t4205 * t20816;
    let t75940 = 144.0_f64 * t67230;
    let t75941 = 144.0_f64 * t67243;
    let t75942 = 0.65061487801810439052e-1_f64 * t58972;
    let t75943 = 16.0_f64 * t67463;
    let t75947 = -36.0_f64 * t17116 * t2522 * t28248 + 12.0_f64 * t1877 * t5664 * t59564 + t39549 + t39563 - t39585 + t39590 + t40799 + t40801 - t40803 + t75939 + t75940 + t75941 + t75942 + t75943;
    (t75939, t75940, t75941, t75942, t75943, t75947)
}
