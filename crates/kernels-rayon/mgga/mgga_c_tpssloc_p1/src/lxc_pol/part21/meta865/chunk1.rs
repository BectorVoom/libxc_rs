//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3158/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3158(t18911: f64, t3411: f64, t1164: f64, t3377: f64, t43689: f64, t43692: f64, t6068: f64, t18271: f64, t18274: f64, t43984: f64, t14967: f64, t4869: f64) -> (f64, f64, f64, f64, f64) {
    let t65314 = 0.34631718211362927518e2_f64 * t3411 * t18911;
    let t65319 = 0.91082604192152556044e5_f64 * t1164 * t43689 * t6068 * t43692 * t3377;
    let t65321 = 0.70178683471615754484e1_f64 * t3411 * t18271;
    let t65324 = 0.10254018858216406658e4_f64 * t1164 * t18274 * t43984;
    let t65326 = 0.69263436422725855034e2_f64 * t4869 * t14967;
    (t65314, t65319, t65321, t65324, t65326)
}
