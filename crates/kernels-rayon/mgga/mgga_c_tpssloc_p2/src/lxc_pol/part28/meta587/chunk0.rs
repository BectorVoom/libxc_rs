//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1879/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1879(t22986: f64, t6646: f64, t829: f64, t87111: f64, t25273: f64, t6579: f64, t244: f64, t268: f64, t6559: f64, t25250: f64, t87202: f64, t25316: f64, t82038: f64) -> (f64, f64, f64, f64, f64) {
    let t87705 = t22986 * t6646 * t87111 * t829;
    let t87709 = t6579 * t25273;
    let t87712 = t6559 * t244 * t268;
    let t87714 = t87712 * t87202 * t25250;
    let t87718 = t82038 * t25316;
    (t87705, t87709, t87712, t87714, t87718)
}
