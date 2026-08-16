//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 992/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk992(t1888: f64, t232: f64, t6646: f64, t92552: f64, t26676: f64, t33384: f64, t6547: f64, t121495: f64, t25038: f64, t25248: f64, t776: f64, t33429: f64) -> (f64, f64, f64, f64, f64) {
    let t121560 = t1888 * t6646 * t92552 * t232;
    let t121563 = t1888 * t6646 * t26676;
    let t121574 = t6547 * t33384;
    let t121612 = t25038 * t25248 * t121495 * t776;
    let t121629 = t6547 * t33429;
    (t121560, t121563, t121574, t121612, t121629)
}
