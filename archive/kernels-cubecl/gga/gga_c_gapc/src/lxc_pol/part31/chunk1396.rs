//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1396/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1396<F: Float>(t34611: F, t34613: F, t34615: F, t34617: F, t34619: F, t34622: F, t34625: F, t34633: F, t34638: F, t34644: F, t34661: F, t34663: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37009 = F::cast_from(0.10117831965157323855e-7_f64) * t34611;
    let t37010 = F::cast_from(0.55015711310542948459e-6_f64) * t34613;
    let t37011 = F::cast_from(0.40483072916666666668e-3_f64) * t34615;
    let t37012 = F::cast_from(0.80966145833333333338e-4_f64) * t34617;
    let t37013 = F::cast_from(0.11594181388521408695e-4_f64) * t34619;
    let t37014 = F::cast_from(0.34752370105806885418e-3_f64) * t34622;
    let t37015 = F::cast_from(0.30890995649606120371e-4_f64) * t34625;
    let t37017 = F::cast_from(0.43440462632258606772e-4_f64) * t34633;
    let t37020 = F::cast_from(0.43440462632258606772e-4_f64) * t34638;
    let t37022 = F::cast_from(0.10793703140429833089e-5_f64) * t34644;
    let t37027 = F::cast_from(0.21720231316129303386e-4_f64) * t34661;
    let t37028 = F::cast_from(0.11372686522837130914e-5_f64) * t34663;
    (t37009, t37010, t37011, t37012, t37013, t37014, t37015, t37017, t37020, t37022, t37027, t37028)
}
