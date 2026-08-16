//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1039/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1039<F: Float>(t142: F, t2948: F, t2973: F, t2981: F, t879: F, t15318: F, t880: F, t881: F, t119: F, t862: F, t2978: F, t157: F, t172: F, t2942: F, t2943: F, t2949: F, t2951: F, t2974: F, t2979: F, t2982: F, t2989: F, t2997: F, t3008: F, t3016: F, t899: F) -> F {
    let t15364 = t142 * t2948;
    let t15369 = t2973 * t2981 * t879;
    let t15375 = t15318 * t880;
    let t15378 = t881 * t2973;
    let t15381 = t119 * t862;
    let t15388 = t142 * t2978;
    let t15401 = F::cast_from(0.10685e0_f64) * t2942 * t15364 * t2951 + F::cast_from(0.48245472966453314466e2_f64) * t2979 * t15369 - F::cast_from(0.32530742648344572643e-1_f64) * t2989 * t157 * t2997 + F::cast_from(6.0_f64) * t2979 * t15375 - F::cast_from(6.0_f64) * t2949 * t15378 + F::cast_from(0.71233333333333333334e-1_f64) * t2942 * t15381 * t881 - F::cast_from(0.53425e-1_f64) * t2942 * t2943 * t2974 - F::cast_from(0.85917146441092277512e0_f64) * t2942 * t15388 * t2982 - F::cast_from(0.21687161765563048428e-1_f64) * t2989 * t172 * t899 + F::cast_from(0.16265371324172286321e-1_f64) * t2989 * t157 * t3008 + F::cast_from(0.48159446095139119799e0_f64) * t2989 * t157 * t3016;
    t15401
}
