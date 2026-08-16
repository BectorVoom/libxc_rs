//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1011/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1011(t35682: f64, t31773: f64, t8634: f64, t2288: f64, t4210: f64, t15386: f64, t31057: f64, t1347: f64, t7614: f64, t1967: f64, t8502: f64, t1998: f64, t5089: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35683 = 0.28582678745379824648e-3_f64 * t35682;
    let t35685 = t31773 * t8634;
    let t35686 = 11.0_f64 / 48.0_f64 * t35685;
    let t35700 = t2288 * t4210;
    let t35702 = t31057 * t15386 * t35700;
    let t35703 = 0.94344276868812456204e-3_f64 * t35702;
    let t35709 = t7614 * t1347;
    let t35710 = 0.32012600194825403606e-1_f64 * t35709;
    let t35722 = t1967 * t8502;
    let t35723 = 0.25724410870841842184e-2_f64 * t35722;
    let t35733 = t1998 * t5089;
    (t35683, t35686, t35700, t35703, t35710, t35723, t35733)
}
