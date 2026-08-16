//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 942/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk942(t2289: f64, t3428: f64, t3430: f64, t2317: f64, t3436: f64, t158: f64, t122: f64, t166: f64, t874: f64, t3434: f64, t502: f64, t58: f64, t875: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10922 = t2289 * t3428;
    let t10923 = t10922 * t3430;
    let t10924 = 0.15243824895787514157e-3_f64 * t10923;
    let t10927 = t3436 * t2317;
    let t10928 = t10927 * t158;
    let t10929 = t166 * t122;
    let t10930 = t10929 * t874;
    let t10932 = t3434 * t10928 * t10930;
    let t10933 = 0.43368970657079495312e-4_f64 * t10932;
    let t10935 = t502 * t875 * t58;
    (t10922, t10924, t10928, t10929, t10930, t10933, t10935)
}
