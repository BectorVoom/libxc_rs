//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 951/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk951(t3473: f64, t661: f64, t1815: f64, t639: f64, t10550: f64, t1809: f64, t10555: f64, t2677: f64, t3465: f64, t617: f64, t1620: f64, t1022: f64, t1044: f64) -> (f64, f64, f64, f64, f64) {
    let t10708 = t3473 * t661;
    let t10709 = t1815 * t10708;
    let t10711 = 4.0_f64 / 45.0_f64 * t639 * t10709;
    let t10712 = t1809 * t10550;
    let t10714 = 8.0_f64 / 45.0_f64 * t639 * t10712;
    let t10715 = t2677 * t10555;
    let t10717 = 4.0_f64 / 27.0_f64 * t639 * t10715;
    let t10718 = t3465 * t617;
    let t10719 = t2677 * t10718;
    let t10721 = 8.0_f64 / 27.0_f64 * t1620 * t10719;
    let t10722 = t1022 * t1044;
    (t10711, t10714, t10717, t10721, t10722)
}
