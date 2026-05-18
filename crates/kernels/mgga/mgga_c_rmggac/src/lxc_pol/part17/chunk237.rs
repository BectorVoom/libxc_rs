//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 237/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk237<F: Float>(t1131: F, t156: F, t155: F, t2: F, t388: F, t428: F, t180: F, t214: F, t243: F, t426: F, t194: F, t231: F) -> (F, F, F, F, F, F, F) {
    let t1132 = t156 * t1131;
    let t1133 = t155 * t1132;
    let t1134 = t388 * t2;
    let t1135 = t1134 * t428;
    let t1138 = t243 * t214 * t180;
    let t1140 = F::new(0.24415263074675393405e-3) * t426 * t1138;
    let t1143 = t194 * t231;
    (t1132, t1133, t1134, t1135, t1138, t1140, t1143)
}
