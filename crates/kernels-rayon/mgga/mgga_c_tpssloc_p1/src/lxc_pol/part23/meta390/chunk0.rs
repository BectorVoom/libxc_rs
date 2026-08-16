//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1194/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1194(t12328: f64, t1815: f64, t12248: f64, t1834: f64, t111: f64, t6470: f64, t2281: f64, t5489: f64, t5465: f64, t2239: f64, t5385: f64, t19681: f64, t2528: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t54793 = t1815 * t12328;
    let t54930 = t12248 * t1834;
    let t55388 = t6470 * t111;
    let t55531 = t2281 * t5489;
    let t55537 = t2281 * t5465;
    let t55921 = t5385 * t2239;
    let t56099 = t19681 * t2528;
    (t54793, t54930, t55388, t55531, t55537, t55921, t56099)
}
