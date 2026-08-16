//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1155/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1155(t35835: f64, t7433: f64, t8912: f64, t1181: f64, t35618: f64, t599: f64, t7337: f64, t7346: f64, t7347: f64, t8480: f64, t31350: f64, t4971: f64) -> (f64, f64, f64, f64, f64) {
    let t35836 = 0.25724410870841842184e-2_f64 * t35835;
    let t35837 = t7433 * t8912;
    let t35838 = 0.12862205435420921092e-2_f64 * t35837;
    let t35841 = t7337 * t1181 * t599 * t35618;
    let t35844 = t7346 * t8480 * t7347;
    let t35845 = 0.21437009059034868486e-3_f64 * t35844;
    let t35846 = t31350 * t4971;
    (t35836, t35838, t35841, t35845, t35846)
}
