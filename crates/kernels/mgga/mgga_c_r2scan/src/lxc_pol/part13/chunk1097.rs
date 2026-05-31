//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1097/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1097<F: Float>(t11511: F, t11513: F, t11516: F, t11520: F, t11524: F, t11526: F, t10637: F, t10639: F, t10640: F, t10665: F, t10671: F, t10690: F, t10917: F, t11029: F, t11167: F, t11169: F) -> F {
    let t39159 = F::cast_from(3.0_f64) * t11511;
    let t39160 = F::cast_from(2.0_f64) * t11513;
    let t39161 = t11516 / F::cast_from(2.0_f64);
    let t39162 = F::cast_from(15.0_f64) / F::cast_from(8.0_f64) * t11520;
    let t39163 = t11524 / F::cast_from(2.0_f64);
    let t39164 = t11526 / F::cast_from(2.0_f64);
    let t39165 = t39159 + t39160 - t10637 + t10639 + t10640 + t11029 + t39161 - t39162 - t10665 + t10671 + t11167 + t11169 - t10690 + t10917 + t39163 + t39164;
    t39165
}
