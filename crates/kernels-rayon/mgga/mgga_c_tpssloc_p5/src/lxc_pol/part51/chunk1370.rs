//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1370/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1370(t1307: f64, t24432: f64, t24995: f64, t33357: f64, t33336: f64, t6876: f64, t115925: f64, t25989: f64, t22574: f64, t32193: f64, t27219: f64, t8526: f64) -> (f64, f64, f64, f64, f64) {
    let t121159 = 6.0_f64 * t24995 * t24432 * t33357 * t1307;
    let t121160 = t6876 * t33336;
    let t121162 = 3.0_f64 * t115925 * t25989;
    let t121165 = 3.0_f64 * t22574 * t32193 * t33357;
    let t121169 = 2.0_f64 * t8526 * t27219;
    (t121159, t121160, t121162, t121165, t121169)
}
