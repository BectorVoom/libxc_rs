//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta865 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3157;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3158;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3159;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta865(t14831: f64, t4869: f64, t18915: f64, t3423: f64, t1164: f64, t14854: f64, t44154: f64, t6068: f64, t18280: f64, t3411: f64, t15041: f64, t11433: f64, t18279: f64, t18911: f64, t3377: f64, t43689: f64, t43692: f64, t18271: f64, t18274: f64, t43984: f64, t14967: f64, t64436: f64, t64441: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t65299, t65301, t65305, t65307, t65309, t65312) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3157(t14831, t4869, t18915, t3423, t1164, t14854, t44154, t6068, t18280, t3411, t15041, t11433, t18279);
        let (t65314, t65319, t65321, t65324, t65326) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3158(t18911, t3411, t1164, t3377, t43689, t43692, t6068, t18271, t18274, t43984, t14967, t4869);
        let t65327 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3159(t64436, t64441, t65299, t65301, t65305, t65307, t65309, t65312, t65314, t65319, t65321, t65324, t65326);
    (t65299, t65301, t65305, t65307, t65309, t65312, t65314, t65319, t65321, t65324, t65326, t65327)
}
