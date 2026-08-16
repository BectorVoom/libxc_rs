//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1056/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1056(t43490: f64, t7427: f64, t7573: f64, t43598: f64, t7572: f64, t10930: f64, t10931: f64, t43494: f64, t33331: f64, t33332: f64, t2660: f64, t33576: f64) -> (f64, f64, f64, f64, f64) {
    let t44053 = t7427 * t7573 * t43490;
    let t44057 = 0.62115540045351614476e2_f64 * t7572 * t7573 * t43598;
    let t44060 = 0.38649669361552115674e3_f64 * t10930 * t10931 * t43494;
    let t44064 = 0.13803453343411469884e3_f64 * t33331 * t33332 * t43494;
    let t44065 = t33576 * t2660;
    (t44053, t44057, t44060, t44064, t44065)
}
