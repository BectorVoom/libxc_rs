//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2299/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2299(t5914: f64, t6703: f64, t5843: f64, t984: f64, t1052: f64, t1635: f64, t18062: f64, t18165: f64, t1955: f64, t1956: f64, t23365: f64, t23588: f64, t25406: f64, t25732: f64, t25738: f64, t25778: f64, t25797: f64, t28474: f64, t28480: f64, t3174: f64, t4660: f64, t4694: f64, t5844: f64, t63215: f64, t6687: f64, t6706: f64, t6771: f64, t89609: f64, t89617: f64, t89666: f64, t986: f64) -> (f64, f64) {
    let t99895 = t6703 * t5914;
    let t99921 = t5843 * t984;
    let t99930 = -0.82246703342411321825e-2_f64 * t6687 * t99895 * t6706 - 0.82246703342411321825e-2_f64 * t6687 * t986 * t28474 + 0.82246703342411321825e-2_f64 * t6687 * t5844 * t23588 - t63215 * t1956 + 2.0_f64 * t6771 * t18062 - 2.0_f64 * t25778 * t4694 + 0.3289868133696452873e-1_f64 * t6687 * t25406 * t25738 - 0.82246703342411321825e-2_f64 * t6687 * t23365 * t28480 - 2.0_f64 * t89666 * t1635 - 0.48738787165873375896e-2_f64 * t89609 - 2.0_f64 * t4660 * t25732 - 0.82246703342411321825e-2_f64 * t6687 * t99921 * t25797 - 0.36554090374405031923e-2_f64 * t89617 + 2.0_f64 * t1052 * t3174 * t1955 * t18165;
    (t99921, t99930)
}
