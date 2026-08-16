//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 991/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk991(t1983: f64, t30127: f64, t7586: f64, t8791: f64, t1181: f64, t21143: f64, t604: f64, t7493: f64, t30786: f64, t30790: f64, t1992: f64, t5606: f64, t7585: f64) -> (f64, f64, f64, f64, f64) {
    let t34957 = t30127 * t7586 * t1983 * t8791;
    let t34961 = t7493 * t1181 * t604 * t21143;
    let t34986 = 0.21437009059034868486e-3_f64 * t30786;
    let t34987 = 0.28582678745379824648e-3_f64 * t30790;
    let t34990 = t7585 * t7586 * t1992 * t5606;
    (t34957, t34961, t34986, t34987, t34990)
}
