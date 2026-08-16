//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1212/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1212(t27543: f64, t5747: f64, t4294: f64, t17341: f64, t28629: f64, t491: f64, t5998: f64, t7953: f64, t17430: f64, t7948: f64, t17436: f64, t28624: f64) -> (f64, f64, f64, f64, f64) {
    let t97767 = t5747 * t27543;
    let t97768 = t97767 * t4294;
    let t97770 = t28629 * t17341;
    let t97772 = t5998 * t491;
    let t97773 = t97772 * t7953;
    let t97775 = t7948 * t17430;
    let t97777 = t28624 * t17436;
    (t97768, t97770, t97773, t97775, t97777)
}
