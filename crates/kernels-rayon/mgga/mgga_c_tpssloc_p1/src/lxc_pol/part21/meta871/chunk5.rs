//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3205/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3205(t11871: f64, t11881: f64, t11888: f64, t1201: f64, t15022: f64, t15247: f64, t15426: f64, t1758: f64, t18301: f64, t19169: f64, t19174: f64, t19197: f64, t3507: f64, t3604: f64, t3610: f64, t3624: f64, t3625: f64, t44724: f64, t44726: f64, t44730: f64, t470: f64, t493: f64, t5011: f64, t5079: f64, t52479: f64, t52480: f64, t6252: f64, t6256: f64, t6260: f64, t65265: f64, t66675: f64) -> f64 {
    let t66702 = 2.0_f64 * t3604 * t19174 - 6.0_f64 * t11888 * t6260 * t15247 + 24.0_f64 * t44724 * t6252 * t44726 * t3507 + 2.0_f64 * t1201 * t19197 + t470 * t493 * t66675 + 24.0_f64 * t52479 * t52480 * t18301 * t5011 + 2.0_f64 * t3610 * t6260 * t11871 - 2.0_f64 * t3624 * t65265 * t3625 + 6.0_f64 * t11881 * t6252 * t44730 - 2.0_f64 * t3624 * t6256 * t15022 - 4.0_f64 * t3624 * t19169 * t5079 - 12.0_f64 * t11888 * t6256 * t15247 + 2.0_f64 * t15426 * t1758;
    t66702
}
