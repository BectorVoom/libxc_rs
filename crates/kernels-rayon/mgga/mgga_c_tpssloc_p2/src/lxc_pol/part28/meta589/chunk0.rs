//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1883/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1883(t25060: f64, t6547: f64, t1880: f64, t23237: f64, t25216: f64, t25192: f64, t81651: f64, t82074: f64, t6552: f64, t6555: f64, t87782: f64, t23270: f64, t25038: f64, t25191: f64, t87036: f64) -> (f64, f64, f64, f64, f64) {
    let t87804 = t6547 * t25060;
    let t87822 = t1880 * t23237 * t25216;
    let t87835 = t81651 * t82074 * t25192;
    let t87861 = t6552 * t87782 * t6555;
    let t87866 = t25038 * t23270 * t25191 * t87036;
    (t87804, t87822, t87835, t87861, t87866)
}
