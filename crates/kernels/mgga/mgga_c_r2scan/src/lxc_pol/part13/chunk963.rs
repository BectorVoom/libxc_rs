//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 963/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk963<F: Float>(t3436: F, t57: F, t875: F, t3439: F, t10978: F, t2333: F, t792: F, t795: F, t3276: F, t3275: F, t10831: F, t1102: F, t1104: F) -> (F, F, F, F, F, F, F, F) {
    let t10979 = t57 * t3436;
    let t10980 = t10979 * t875;
    let t10981 = t10980 * t3439;
    let t10982 = t10978 * t10981;
    let t10983 = F::cast_from(0.43368970657079495312e-4_f64) * t10982;
    let t10984 = t2333 * t792;
    let t10985 = t10984 * t795;
    let t10986 = t3276 * t10985;
    let t10987 = t3275 * t10986;
    let t10988 = F::new(5.0) / F::new(8.0) * t10987;
    let t10990 = t1102 * t10831 * t1104;
    (t10979, t10980, t10981, t10983, t10985, t10986, t10988, t10990)
}
