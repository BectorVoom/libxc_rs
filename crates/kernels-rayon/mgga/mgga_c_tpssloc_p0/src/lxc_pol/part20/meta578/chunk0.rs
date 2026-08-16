//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2142/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2142(t10870: f64, t3117: f64, t1020: f64, t10858: f64, t248: f64, t3101: f64, t10961: f64, t3108: f64, t10423: f64, t10937: f64, t2955: f64, t3158: f64) -> (f64, f64, f64, f64, f64) {
    let t43114 = t3117 * t10870;
    let t43118 = t1020 * t248 * t3101 * t10858;
    let t43120 = t10961 * t3108;
    let t43143 = t10937 * t10423;
    let t43155 = t2955 * t3158;
    (t43114, t43118, t43120, t43143, t43155)
}
