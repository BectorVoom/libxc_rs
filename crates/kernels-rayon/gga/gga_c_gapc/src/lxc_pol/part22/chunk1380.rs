//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1380/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1380(t33784: f64, t33787: f64, t33789: f64, t33791: f64, t33793: f64, t33796: f64, t33779: f64, t36698: f64, t36699: f64, t36700: f64, t36701: f64, t33801: f64, t33803: f64, t33808: f64, t33810: f64, t33812: f64, t33815: f64, t33818: f64, t33820: f64, t33823: f64, t33825: f64, t33828: f64) -> (f64, f64) {
    let t36703 = 0.19336232562226912508e-8_f64 * t33784;
    let t36704 = 0.2845640240200497334e-7_f64 * t33787;
    let t36705 = 0.34782544165564226085e-4_f64 * t33789;
    let t36706 = 0.42205124476153752644e-7_f64 * t33791;
    let t36707 = 0.33764099580923002116e-6_f64 * t33793;
    let t36708 = 0.21102562238076876322e-7_f64 * t33796;
    let t36709 = -t36698 - t36699 - t36700 + t36701 - 0.57970906942607043474e-5_f64 * t33779 - t36703 + t36704 + t36705 - t36706 + t36707 + t36708;
    let t36722 = 0.40094868252346065012e-6_f64 * t33801 - 0.21102562238076876322e-7_f64 * t33803 - 0.22098551499687900008e-7_f64 * t33808 - 0.55015711310542948459e-6_f64 * t33810 + 0.40481770833333333336e-4_f64 * t33812 + 0.57920616843011475696e-5_f64 * t33815 - 0.50680539737635041234e-3_f64 * t33818 - 0.34752370105806885418e-3_f64 * t33820 + 0.57920616843011475696e-5_f64 * t33823 - 0.50680539737635041234e-3_f64 * t33825 - 0.34752370105806885418e-3_f64 * t33828;
    (t36709, t36722)
}
