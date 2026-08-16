//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1850/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1850(t93134: f64, t95546: f64, t26435: f64, t9303: f64, t2470: f64, t26543: f64, t7058: f64, t7385: f64, t9292: f64, t2435: f64, t26447: f64, t10509: f64, t26481: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t95567 = 0.43639970290213137151e-3_f64 * t93134 * t95546;
    let t95569 = 0.26019841438354088051e-2_f64 * t9303 * t26435;
    let t95575 = t26543 * t2470;
    let t95576 = t7058 * t95575;
    let t95607 = 0.17073386770573548589e-1_f64 * t9292 * t7385;
    let t95620 = t2435 * t26447;
    let t95628 = t26481 * t10509;
    (t95567, t95569, t95575, t95576, t95607, t95620, t95628)
}
