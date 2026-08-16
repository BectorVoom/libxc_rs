//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2500/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2500(t50903: f64, t50905: f64, t50907: f64, t50912: f64, t50917: f64, t50919: f64, t50921: f64, t50926: f64, t50931: f64, t50934: f64, t50937: f64, t50940: f64) -> f64 {
    let t50942 = -0.12077e1_f64 * t50903 - 0.60385e0_f64 * t50905 - 0.181155e1_f64 * t50907 + 0.10064166666666666666e1_f64 * t50912 + 0.40256666666666666666e1_f64 * t50917 - 0.26837777777777777778e0_f64 * t50919 - 0.33547222222222222222e0_f64 * t50921 - 0.89459259259259259259e0_f64 * t50926 + 0.181155e1_f64 * t50931 + 0.181155e1_f64 * t50934 + 0.543465e1_f64 * t50937 + 0.60385e0_f64 * t50940;
    t50942
}
