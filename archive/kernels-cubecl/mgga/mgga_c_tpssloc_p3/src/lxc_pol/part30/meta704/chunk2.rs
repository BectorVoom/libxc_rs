//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2299/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2299<F: Float>(t5914: F, t6703: F, t5843: F, t984: F, t1052: F, t1635: F, t18062: F, t18165: F, t1955: F, t1956: F, t23365: F, t23588: F, t25406: F, t25732: F, t25738: F, t25778: F, t25797: F, t28474: F, t28480: F, t3174: F, t4660: F, t4694: F, t5844: F, t63215: F, t6687: F, t6706: F, t6771: F, t89609: F, t89617: F, t89666: F, t986: F) -> (F, F) {
    let t99895 = t6703 * t5914;
    let t99921 = t5843 * t984;
    let t99930 = -F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t99895 * t6706 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t986 * t28474 + F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t5844 * t23588 - t63215 * t1956 + F::cast_from(2.0_f64) * t6771 * t18062 - F::cast_from(2.0_f64) * t25778 * t4694 + F::cast_from(0.3289868133696452873e-1_f64) * t6687 * t25406 * t25738 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t23365 * t28480 - F::cast_from(2.0_f64) * t89666 * t1635 - F::cast_from(0.48738787165873375896e-2_f64) * t89609 - F::cast_from(2.0_f64) * t4660 * t25732 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t99921 * t25797 - F::cast_from(0.36554090374405031923e-2_f64) * t89617 + F::cast_from(2.0_f64) * t1052 * t3174 * t1955 * t18165;
    (t99921, t99930)
}
