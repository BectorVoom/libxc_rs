//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1394/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1394(t34611: f64, t34613: f64, t34615: f64, t34617: f64, t34619: f64, t34622: f64, t34625: f64, t34633: f64, t34638: f64, t34644: f64, t34661: f64, t34663: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37009 = 0.10117831965157323855e-7_f64 * t34611;
    let t37010 = 0.55015711310542948459e-6_f64 * t34613;
    let t37011 = 0.40483072916666666668e-3_f64 * t34615;
    let t37012 = 0.80966145833333333338e-4_f64 * t34617;
    let t37013 = 0.11594181388521408695e-4_f64 * t34619;
    let t37014 = 0.34752370105806885418e-3_f64 * t34622;
    let t37015 = 0.30890995649606120371e-4_f64 * t34625;
    let t37017 = 0.43440462632258606772e-4_f64 * t34633;
    let t37020 = 0.43440462632258606772e-4_f64 * t34638;
    let t37022 = 0.10793703140429833089e-5_f64 * t34644;
    let t37027 = 0.21720231316129303386e-4_f64 * t34661;
    let t37028 = 0.11372686522837130914e-5_f64 * t34663;
    (t37009, t37010, t37011, t37012, t37013, t37014, t37015, t37017, t37020, t37022, t37027, t37028)
}
