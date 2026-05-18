//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1083/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1083<F: Float>(t11481: F, t11484: F, t11488: F, t11491: F, t11494: F, t11499: F, t11503: F, t11507: F, t11511: F, t11513: F, t11516: F, t11520: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t39150 = t11481 / F::new(2.0);
    let t39151 = t11484 / F::new(2.0);
    let t39152 = F::new(15.0) / F::new(8.0) * t11488;
    let t39153 = F::new(3.0) / F::new(2.0) * t11491;
    let t39154 = t11494 / F::new(2.0);
    let t39155 = F::new(3.0) / F::new(2.0) * t11499;
    let t39156 = F::new(3.0) / F::new(2.0) * t11503;
    let t39157 = F::new(3.0) / F::new(2.0) * t11507;
    let t39159 = F::new(3.0) * t11511;
    let t39160 = F::new(2.0) * t11513;
    let t39161 = t11516 / F::new(2.0);
    let t39162 = F::new(15.0) / F::new(8.0) * t11520;
    (t39150, t39151, t39152, t39153, t39154, t39155, t39156, t39157, t39159, t39160, t39161, t39162)
}
