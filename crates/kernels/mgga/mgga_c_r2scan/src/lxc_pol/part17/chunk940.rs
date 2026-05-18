//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 940/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk940<F: Float>(t10899: F, t3320: F, t783: F, t774: F, t787: F, t2289: F, t3428: F, t3430: F, t2317: F, t3436: F, t158: F, t122: F, t166: F) -> (F, F, F, F, F, F, F) {
    let t10901 = t783 * t10899 * t3320;
    let t10903 = t774 * t787;
    let t10905 = t783 * t10903 * t3320;
    let t10906 = F::new(0.46574606203128791246e-1) * t10905;
    let t10922 = t2289 * t3428;
    let t10923 = t10922 * t3430;
    let t10927 = t3436 * t2317;
    let t10928 = t10927 * t158;
    let t10929 = t166 * t122;
    (t10901, t10903, t10906, t10922, t10923, t10928, t10929)
}
