//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1130/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1130<F: Float>(t40228: F, t40251: F, t40779: F, t40788: F, t40808: F, t12382: F, t12386: F, t12388: F, t12394: F, t12581: F, t12583: F, t12584: F, t39149: F, t39150: F, t39151: F, t39152: F, t39153: F, t39154: F, t39155: F, t39156: F) -> (F, F, F, F, F, F) {
    let t41753 = F::new(0.32524801797942610062e-3) * t40228;
    let t41770 = F::new(0.35707476898255463229e0) * t40251;
    let t41858 = F::new(22.0) / F::new(9.0) * t40779;
    let t41864 = F::new(44.0) / F::new(9.0) * t40788;
    let t41872 = F::new(22.0) / F::new(9.0) * t40808;
    let t42376 = t12382 - t39149 + t12386 - t39150 + t39151 - t39152 - t12388 + t12581 + t39153 - t39154 + t12583 + t12584 - t12394 - t39155 + t39156;
    (t41753, t41770, t41858, t41864, t41872, t42376)
}
