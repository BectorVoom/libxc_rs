//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1795/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1795(t236: f64, t25093: f64, t87229: f64, t87230: f64, t81764: f64, t1512: f64, t81807: f64, t81824: f64, t23041: f64, t4236: f64, t23040: f64, t4166: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t87233 = t87229 * t87230 * t236 * t25093;
    let t87237 = 119.0_f64 / 864.0_f64 * t81764;
    let t87243 = t81807 * t1512;
    let t87247 = t81824 * t1512;
    let t87255 = t23041 * t4236;
    let t87261 = t4166 * t23040;
    (t87233, t87237, t87243, t87247, t87255, t87261)
}
