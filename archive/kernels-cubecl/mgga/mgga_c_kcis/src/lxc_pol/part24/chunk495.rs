//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 495/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk495<F: Float>(t169: F, t4532: F, t234: F, t1071: F, t359: F, zeta_threshold: F) -> (F, F, F) {
    let t170 = t169 <= zeta_threshold;
    let t4533 = piecewise3::<F>(t170, F::cast_from(0.0_f64), t4532);
    let t4534 = t234 * t4533;
    let t4546 = t359 * t1071;
    (t4533, t4534, t4546)
}
