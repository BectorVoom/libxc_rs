//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 857/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk857<F: Float>(t10844: F, t2201: F, t3319: F, t3320: F, t481: F, t2207: F, t2161: F, t2164: F, t505: F, t502: F, t57: F, t512: F) -> (F, F, F, F, F, F) {
    let t10846 = t2201 * t3319 * t10844;
    let t10848 = t3320 * t481;
    let t10850 = t2207 * t3319 * t10848;
    let t10853 = t2161 * t505 * t2164;
    let t10854 = 0.81312004494856525156e-4 * t10853;
    let t10855 = t502 * t57;
    let t10856 = t512 * t10855;
    (t10846, t10848, t10850, t10854, t10855, t10856)
}
