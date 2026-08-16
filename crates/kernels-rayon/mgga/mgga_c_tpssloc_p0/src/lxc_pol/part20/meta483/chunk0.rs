//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1968/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1968(t16048: f64, t5335: f64, t3793: f64, t1332: f64, t5333: f64, t5230: f64, t68: f64) -> (f64, f64, f64, f64) {
    let t16049 = t5335 * t16048;
    let t16052 = t5335 * t3793;
    let t16055 = t1332 * t5333;
    let t16060 = t5230 * t68;
    (t16049, t16052, t16055, t16060)
}
