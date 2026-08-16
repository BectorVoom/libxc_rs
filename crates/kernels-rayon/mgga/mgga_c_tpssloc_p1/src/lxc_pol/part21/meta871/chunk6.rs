//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3206/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3206(t11871: f64, t11888: f64, t1244: f64, t1246: f64, t15001: f64, t15019: f64, t15027: f64, t15032: f64, t15245: f64, t15253: f64, t15257: f64, t19129: f64, t19179: f64, t3507: f64, t3590: f64, t3604: f64, t3610: f64, t44698: f64, t44701: f64, t44741: f64, t45320: f64, t4978: f64, t5011: f64, t5068: f64, t5073: f64, t52480: f64, t53613: f64, t53646: f64, t6218: f64, t6252: f64, t6253: f64, t6256: f64) -> f64 {
    let t66737 = -6.0_f64 * t11888 * t6252 * t44741 - 36.0_f64 * t44698 * t6252 * t44701 * t3507 - 24.0_f64 * t53646 * t52480 * t4978 * t5011 + 12.0_f64 * t53613 * t15001 + 4.0_f64 * t3610 * t6256 * t11871 + 2.0_f64 * t45320 * t6253 - 2.0_f64 * t15245 * t15019 + t1244 * t3590 * t6218 * t1246 - 4.0_f64 * t15245 * t15257 + 4.0_f64 * t15027 * t15253 + 8.0_f64 * t3610 * t19179 * t5068 + 4.0_f64 * t15032 * t5073 + 2.0_f64 * t3604 * t19129;
    t66737
}
