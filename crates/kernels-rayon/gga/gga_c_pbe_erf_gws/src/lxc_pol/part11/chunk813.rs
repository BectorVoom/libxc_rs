//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 813/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk813(t13005: f64, t13006: f64, t13013: f64, t13016: f64, t13021: f64, t13023: f64, t13025: f64, t13042: f64, t10283: f64, t11157: f64, t11160: f64, t11166: f64, t12381: f64, t12891: f64, t153: f64, t156: f64, t168: f64, t242: f64, t245: f64, t4550: f64, t4557: f64, t4600: f64, t5588: f64, t5592: f64, t5595: f64, t7981: f64, t8042: f64, t8051: f64, t8058: f64, t8066: f64) -> (f64, f64) {
    let t13045 = t13005 + t13006 + t13013 + t13016 + t13021 + t13023 + t13025 + t13042;
    let t13055 = -0.25128846160651320563e0_f64 * t11157 + 0.25128846160651320563e0_f64 * t11160 - t4550 + t4557 - t4600 - 0.50257692321302641125e0_f64 * t8042 + 0.42708890021612718669e0_f64 * t153 * t156 * t12381 + t5588 + 0.50257692321302641125e0_f64 * t8051 + 0.39861630686838537423e1_f64 * t7981 - 0.11938374665504764976e-1_f64 * t168 * t245 * t13045 - 0.25128846160651320563e0_f64 * t8058 + t5592 - 0.15917832887339686635e0_f64 * t8066 - t5595 + 0.59691873327523824879e-1_f64 * t11166 - 0.17083556008645087467e1_f64 * t10283 - 0.83762820535504401876e-1_f64 * t12891 * t242;
    (t13045, t13055)
}
