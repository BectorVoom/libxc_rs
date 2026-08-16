//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1132/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1132<F: Float>(t12058: F, t12061: F, t12063: F, t12228: F, t12081: F, t12084: F, t12087: F, t12092: F, t12095: F, t12100: F, t12103: F, t12109: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t41129 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t12058;
    let t41130 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t12061;
    let t41131 = F::cast_from(2.0_f64) * t12063;
    let t41132 = F::cast_from(2.0_f64) * t12228;
    let t41133 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t12081;
    let t41134 = t12084 / F::cast_from(2.0_f64);
    let t41135 = t12087 / F::cast_from(2.0_f64);
    let t41138 = t12092 / F::cast_from(2.0_f64);
    let t41139 = F::cast_from(15.0_f64) / F::cast_from(8.0_f64) * t12095;
    let t41140 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t12100;
    let t41141 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t12103;
    let t41142 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t12109;
    (t41129, t41130, t41131, t41132, t41133, t41134, t41135, t41138, t41139, t41140, t41141, t41142)
}
