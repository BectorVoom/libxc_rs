//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1035/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1035<F: Float>(t22600: F, t1790: F, t1792: F, t21875: F, t10195: F, t10345: F, t1807: F, t1809: F, t1824: F, t1848: F, t1867: F, t209: F, t22075: F, t22120: F, t22148: F, t22411: F, t22445: F, t22497: F, t22508: F, t22510: F, t22513: F, t22516: F, t22522: F, t22524: F, t22526: F, t22528: F, t22531: F, t22562: F, t22563: F, t22566: F, t22578: F, t22581: F, t22593: F, t22598: F, t508: F, t566: F, t572: F, t581: F, t587: F, t62: F, t6387: F, t6391: F, t6392: F, t6405: F, t6406: F, t6408: F, t6425: F, t6427: F, t75: F) -> (F, F, F) {
    let t22601 = F::new(1.0) / t22600;
    let t22610 = F::new(0.48245472966453314466e2) * t1790 * t21875 * t1792;
    let t22617 = F::new(0.38527556876111295841e1) * t209 * t508 * t6405 * t6408 - F::new(0.14172186339420759129e3) * t209 * t508 * t6387 * t6392 + t22497 - F::new(0.12304676425209353917e5) * t75 * t22148 * t22120 * t6427 + F::new(0.58482233974552040708e0) * t581 * t22531 * t587 + F::new(1.0) * t566 * (-F::new(0.39219166666666666667e1) * t22508 + F::new(0.376504e2) * t22510 - F::new(0.13944592592592592593e2) * t22513 + F::new(0.12201518518518518519e2) * t22516 + F::new(0.5356037037037037037e1) * t10195 + F::new(0.14025833333333333333e0) * t22522 - F::new(0.22441333333333333332e1) * t22524 + F::new(0.24934814814814814815e1) * t22526 + F::new(0.21817962962962962963e1) * t22528 + F::new(0.16979925925925925926e1) * t10345) * t572 - t22562 + F::new(0.19965908856856833625e6) * t62 / t22563 * t22411 / t22566 - t22578 - t22581 - F::new(0.24829604254387158296e5) * t62 / t1824 / t1807 * t22411 * t6391 + t22593 - F::new(6.0) * t1809 * t22445 * t572 + F::new(0.91080982599109921211e5) * t75 * t22598 * t22120 * t22601 + F::new(0.6233672123775310788e3) * t6425 * t22120 * t1867 - t22610 - F::new(0.35089340384731224426e1) * t1848 * t22075 * t587 - F::new(0.1403573615389248977e2) * t6406 * t22120 * t587;
    (t22601, t22610, t22617)
}
