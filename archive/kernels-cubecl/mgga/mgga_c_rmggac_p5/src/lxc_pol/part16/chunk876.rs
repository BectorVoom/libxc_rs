//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 876/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk876<F: Float>(t2227: F, t558: F, t1587: F, t698: F, t41523: F, t41531: F, t41534: F, t41536: F, t41549: F, t42144: F, t42151: F, t42166: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t44232 = t2227 * t558;
    let t44239 = t698 * t1587;
    let t44337 = F::cast_from(0.47896966807455234256e0_f64) * t41523;
    let t44339 = F::cast_from(0.95793933614910468512e0_f64) * t41531;
    let t44340 = F::cast_from(0.19158786722982093702e1_f64) * t41534;
    let t44341 = F::cast_from(0.47896966807455234256e0_f64) * t41536;
    let t44362 = F::cast_from(0.3193131120497015617e0_f64) * t41549;
    let t44382 = F::cast_from(0.49658699875514145965e-4_f64) * t42144;
    let t44385 = F::cast_from(0.47896966807455234256e0_f64) * t42151;
    let t44396 = F::cast_from(0.21819729323396273384e0_f64) * t42166;
    (t44232, t44239, t44337, t44339, t44340, t44341, t44362, t44382, t44385, t44396)
}
