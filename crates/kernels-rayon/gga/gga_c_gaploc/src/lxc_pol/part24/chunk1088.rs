//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1088/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1088(t27835: f64, t286: f64, t708: f64, t1236: f64, t129: f64, t1687: f64, t6371: f64, t9102: f64, t5337: f64, t5340: f64, t15660: f64, t4066: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27837 = t27835 * t286 * t708;
    let t27839 = t129 * t1236;
    let t27840 = t27839 * t1687;
    let t27842 = t9102 * t6371;
    let t27844 = t27842 * t5337 * t5340;
    let t27846 = 1.0_f64 / t15660;
    let t27847 = t4066 * t27846;
    (t27837, t27839, t27840, t27842, t27844, t27846, t27847)
}
