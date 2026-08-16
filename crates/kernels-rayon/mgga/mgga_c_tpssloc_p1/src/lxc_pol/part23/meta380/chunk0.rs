//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1182/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1182(t10523: f64, t1573: f64, t10629: f64, t10701: f64, t1543: f64, t10810: f64, t1561: f64, t10770: f64, t10660: f64, t10402: f64, t14618: f64, t14608: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t49099 = t1573 * t10523;
    let t49104 = t1573 * t10629;
    let t49274 = t1543 * t10701;
    let t49285 = t1561 * t10810;
    let t49430 = t1561 * t10770;
    let t49489 = t1543 * t10660;
    let t49929 = t14618 * t10402;
    let t49934 = t14608 * t10402;
    (t49099, t49104, t49274, t49285, t49430, t49489, t49929, t49934)
}
