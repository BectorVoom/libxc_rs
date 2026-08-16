//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3069/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3069<F: Float>(t11185: F, t18262: F, t14913: F, t3313: F, t4785: F, t18266: F, t43964: F, t11275: F, t18265: F, t3307: F, t3265: F, t44075: F, t44077: F, t5988: F) -> (F, F, F, F, F) {
    let t63717 = F::cast_from(0.64327917994770140268e2_f64) * t11185 * t18262;
    let t63720 = F::cast_from(0.32163958997385070134e2_f64) * t3313 * t4785 * t14913;
    let t63722 = F::cast_from(0.1034520258385468006e4_f64) * t43964 * t18266;
    let t63725 = F::cast_from(0.51726012919273400301e3_f64) * t11275 * t18265 * t3307;
    let t63729 = F::cast_from(0.24955700379505800916e5_f64) * t44075 * t5988 * t44077 * t3265;
    (t63717, t63720, t63722, t63725, t63729)
}
