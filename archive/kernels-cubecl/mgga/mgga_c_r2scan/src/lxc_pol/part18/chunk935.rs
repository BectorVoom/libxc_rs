//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 935/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk935<F: Float>(t2228: F, t57: F, t2116: F, t3320: F, t560: F, t2201: F, t3319: F, t481: F, t2207: F, t2161: F, t2164: F, t505: F) -> (F, F, F, F, F, F, F) {
    let t10841 = t2228 * t57;
    let t10842 = t10841 * t2116;
    let t10843 = F::cast_from(0.16463622957338778997e-1_f64) * t10842;
    let t10844 = t3320 * t560;
    let t10846 = t2201 * t3319 * t10844;
    let t10848 = t3320 * t481;
    let t10850 = t2207 * t3319 * t10848;
    let t10853 = t2161 * t505 * t2164;
    (t10841, t10843, t10844, t10846, t10848, t10850, t10853)
}
