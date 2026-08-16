//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1361/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1361(t15868: f64, t1983: f64, t8640: f64, t1307: f64, t24432: f64, t24995: f64, t33357: f64, t33336: f64, t6876: f64, t115925: f64, t25989: f64, t22574: f64, t32193: f64) -> (f64, f64, f64, f64, f64) {
    let t121144 = t1983 * t8640 * t15868;
    let t121159 = 6.0_f64 * t24995 * t24432 * t33357 * t1307;
    let t121160 = t6876 * t33336;
    let t121162 = 3.0_f64 * t115925 * t25989;
    let t121165 = 3.0_f64 * t22574 * t32193 * t33357;
    (t121144, t121159, t121160, t121162, t121165)
}
