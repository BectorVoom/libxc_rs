//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 749/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk749(t5396: f64, t7069: f64, t286: f64, t6361: f64, t708: f64, t1687: f64, t6365: f64, t5337: f64, t5340: f64, t6372: f64, t5345: f64, t5348: f64) -> (f64, f64, f64, f64, f64) {
    let t7070 = t5396 * t7069;
    let t7088 = t6361 * t286 * t708;
    let t7090 = t6365 * t1687;
    let t7093 = t6372 * t5337 * t5340;
    let t7096 = t5345 * t6372 * t5348;
    (t7070, t7088, t7090, t7093, t7096)
}
