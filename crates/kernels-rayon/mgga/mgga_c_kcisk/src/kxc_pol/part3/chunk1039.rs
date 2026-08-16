//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1039/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1039(t142: f64, t2948: f64, t2973: f64, t2981: f64, t879: f64, t15318: f64, t880: f64, t881: f64, t119: f64, t862: f64, t2978: f64, t157: f64, t172: f64, t2942: f64, t2943: f64, t2949: f64, t2951: f64, t2974: f64, t2979: f64, t2982: f64, t2989: f64, t2997: f64, t3008: f64, t3016: f64, t899: f64) -> f64 {
    let t15364 = t142 * t2948;
    let t15369 = t2973 * t2981 * t879;
    let t15375 = t15318 * t880;
    let t15378 = t881 * t2973;
    let t15381 = t119 * t862;
    let t15388 = t142 * t2978;
    let t15401 = 0.10685e0_f64 * t2942 * t15364 * t2951 + 0.48245472966453314466e2_f64 * t2979 * t15369 - 0.32530742648344572643e-1_f64 * t2989 * t157 * t2997 + 6.0_f64 * t2979 * t15375 - 6.0_f64 * t2949 * t15378 + 0.71233333333333333334e-1_f64 * t2942 * t15381 * t881 - 0.53425e-1_f64 * t2942 * t2943 * t2974 - 0.85917146441092277512e0_f64 * t2942 * t15388 * t2982 - 0.21687161765563048428e-1_f64 * t2989 * t172 * t899 + 0.16265371324172286321e-1_f64 * t2989 * t157 * t3008 + 0.48159446095139119799e0_f64 * t2989 * t157 * t3016;
    t15401
}
