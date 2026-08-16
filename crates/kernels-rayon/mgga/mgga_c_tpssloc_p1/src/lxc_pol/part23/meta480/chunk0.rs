//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1437/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1437(t18915: f64, t6102: f64, t6274: f64, t3313: f64, t5989: f64, t6020: f64, t1703: f64, t71231: f64, t14838: f64, t21895: f64, t14850: f64, t21899: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t78344 = 0.35089341735807877242e1_f64 * t18915 * t6102;
    let t78348 = t6274 * t6274;
    let t78355 = 36.0_f64 * t3313 * t5989 * t6020;
    let t78357 = 0.23392894490538584828e1_f64 * t71231 * t1703;
    let t78359 = 24.0_f64 * t14838 * t21895;
    let t78361 = 0.1929837539843104208e3_f64 * t14850 * t21899;
    (t78344, t78348, t78355, t78357, t78359, t78361)
}
