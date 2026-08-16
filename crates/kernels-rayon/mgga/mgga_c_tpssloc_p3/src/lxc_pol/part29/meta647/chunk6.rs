//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2148/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2148(t25038: f64, t25248: f64, t776: f64, t87130: f64, t22986: f64, t6646: f64, t829: f64, t87111: f64, t82039: f64, t25273: f64, t6579: f64, t244: f64, t268: f64, t6559: f64) -> (f64, f64, f64, f64, f64) {
    let t87699 = t25038 * t25248 * t87130 * t776;
    let t87705 = t22986 * t6646 * t87111 * t829;
    let t87708 = 0.10417915756705434098e0_f64 * t82039;
    let t87709 = t6579 * t25273;
    let t87710 = 0.38381794893125283518e-1_f64 * t87709;
    let t87712 = t6559 * t244 * t268;
    (t87699, t87705, t87708, t87710, t87712)
}
