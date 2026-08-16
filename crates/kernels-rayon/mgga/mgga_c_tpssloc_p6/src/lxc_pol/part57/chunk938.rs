//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 938/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk938(t214: f64, t7823: f64, t225: f64, t33412: f64, t33371: f64, t6547: f64, t33458: f64, t6579: f64, t23185: f64, t33457: f64, t82074: f64, t33414: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t121401 = t214 * t7823;
    let t121405 = t33412 * t225;
    let t121431 = t6547 * t33371;
    let t121437 = t6579 * t33458;
    let t121444 = t23185 * t82074 * t33457;
    let t121454 = t33414 * t225;
    (t121401, t121405, t121431, t121437, t121444, t121454)
}
