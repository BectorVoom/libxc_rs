//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3069/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3069(t11185: f64, t18262: f64, t14913: f64, t3313: f64, t4785: f64, t18266: f64, t43964: f64, t11275: f64, t18265: f64, t3307: f64, t3265: f64, t44075: f64, t44077: f64, t5988: f64) -> (f64, f64, f64, f64, f64) {
    let t63717 = 0.64327917994770140268e2_f64 * t11185 * t18262;
    let t63720 = 0.32163958997385070134e2_f64 * t3313 * t4785 * t14913;
    let t63722 = 0.1034520258385468006e4_f64 * t43964 * t18266;
    let t63725 = 0.51726012919273400301e3_f64 * t11275 * t18265 * t3307;
    let t63729 = 0.24955700379505800916e5_f64 * t44075 * t5988 * t44077 * t3265;
    (t63717, t63720, t63722, t63725, t63729)
}
