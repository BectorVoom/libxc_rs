//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 290/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk290<F: Float>(t44: F, t51: F, t538: F, t921: F, t529: F, t889: F, t99: F, t101: F, t893: F, zeta_threshold: F) -> (F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t52 = t51 <= zeta_threshold;
    let t927 = t538 * t921;
    let t928 = t529 * t927;
    let t933 = piecewise3::<F>(t45, F::new(0.0), F::new(5.0) / F::new(3.0) * t99 * t889);
    let t936 = piecewise3::<F>(t52, F::new(0.0), F::new(5.0) / F::new(3.0) * t101 * t893);
    let t938 = t933 / F::new(2.0) + t936 / F::new(2.0);
    (t927, t928, t938)
}
