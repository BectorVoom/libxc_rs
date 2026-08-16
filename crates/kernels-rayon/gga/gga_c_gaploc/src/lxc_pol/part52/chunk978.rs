//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 978/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk978(t1022: f64, t12250: f64, t1457: f64, t2009: f64, t2021: f64, t2103: f64, t43994: f64, t44001: f64, t44004: f64, t44009: f64, t45803: f64, t45806: f64, t45809: f64, t45812: f64, t45817: f64, t45820: f64, t45823: f64, t45826: f64, t45831: f64, t45837: f64, t45848: f64, t45856: f64, t45863: f64, t47450: f64, t50092: f64) -> f64 {
    let t50272 = t12250 * t1022;
    let t50276 = t45803 + t45806 + t45809 + t45812 - t45817 - t45820 + t45823 - 0.9585731488480187419e0_f64 * t45826 - t45831 + t45837 - 0.11916829983950142223e0_f64 * t47450 + t45848 + 0.14300195980740170668e1_f64 * t2103 * t1457 * t50092 - t45856 - t43994 + 0.63904876589867916127e-1_f64 * t44001 + 0.38342925953920749676e1_f64 * t44004 + 0.63904876589867916127e-1_f64 * t44009 - 0.71500979903700853338e0_f64 * t2021 * t50272 * t2009 - t45863;
    t50276
}
