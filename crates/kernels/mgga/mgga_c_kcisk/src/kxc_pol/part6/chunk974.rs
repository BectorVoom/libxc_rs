//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 974/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk974<F: Float>(t222: F, t30162: F, t30170: F, t44: F, t291: F, t2071: F, t8459: F, t294: F, t30158: F, t295: F, t559: F, t2231: F, t7706: F, zeta_threshold: F) -> (F, F, F, F) {
    let t223 = t222 <= zeta_threshold;
    let t30172 = (t30162 + t30170) * t44;
    let t30173 = t30172 * t291;
    let t30174 = t2071 * t8459;
    let t30175 = t294 * t30174;
    let t30176 = F::new(3.0) / F::new(16.0) * t30175;
    let t30177 = piecewise3::<F>(t223, F::new(0.0), t30158);
    let t30178 = t295 * t30177;
    let t30179 = t30178 * t559;
    let t30180 = t294 * t30179;
    let t30181 = t30180 / F::new(16.0);
    let t30184 = t7706 * t2231;
    (t30173, t30176, t30181, t30184)
}
