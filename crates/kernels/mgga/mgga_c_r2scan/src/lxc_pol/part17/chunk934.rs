//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 934/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk934<F: Float>(t10839: F, t2228: F, t57: F, t2116: F, t3320: F, t560: F, t2201: F, t3319: F, t481: F, t2207: F, t2161: F, t2164: F, t505: F) -> (F, F, F, F, F, F, F, F) {
    let t10840 = F::new(0.23115257973478049502e0) * t10839;
    let t10841 = t2228 * t57;
    let t10842 = t10841 * t2116;
    let t10844 = t3320 * t560;
    let t10846 = t2201 * t3319 * t10844;
    let t10847 = F::new(0.46574606203128791246e-1) * t10846;
    let t10848 = t3320 * t481;
    let t10850 = t2207 * t3319 * t10848;
    let t10851 = F::new(0.13972381860938637374e0) * t10850;
    let t10853 = t2161 * t505 * t2164;
    (t10840, t10841, t10842, t10844, t10847, t10848, t10851, t10853)
}
