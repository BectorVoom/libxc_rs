//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1006/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1006(t10578: f64, t2630: f64, t2629: f64, t9866: f64, t9575: f64, t9572: f64, t177: f64, t2390: f64, t762: f64, t10575: f64, t10577: f64, t9514: f64, t9517: f64, t9521: f64, t9524: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10579 = t10578 * t2630;
    let t10580 = 0.32530743900905219526e-1_f64 * t10579;
    let t10582 = 0.48159733137676571078e0_f64 * t2629 * t9866;
    let t10584 = 0.21687162600603479684e-1_f64 * t2629 * t9575;
    let t10586 = 0.32530743900905219526e-1_f64 * t2629 * t9572;
    let t10587 = t2390 * t177;
    let t10588 = t10587 * t762;
    let t10589 = 0.17544670867903938621e1_f64 * t10588;
    let t10590 = -t10575 + t9514 - t9517 - t9521 + t10577 + t10580 + t10582 - t10584 - t10586 - t9524 - t10589;
    (t10580, t10582, t10584, t10586, t10587, t10589, t10590)
}
