//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2642/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2642(t16391: f64, t16398: f64, t12283: f64, t16244: f64, t3862: f64, t5231: f64, t16356: f64, t3726: f64, t12328: f64, t1815: f64, t16397: f64, t3777: f64, t5252: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54750 = t16398 * t16391;
    let t54764 = t12283 * t16244;
    let t54785 = t5231 * t3862;
    let t54787 = t3726 * t16356;
    let t54793 = t1815 * t12328;
    let t54801 = t3777 * t16397 * t5252;
    (t54750, t54764, t54785, t54787, t54793, t54801)
}
