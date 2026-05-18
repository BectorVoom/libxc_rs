//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1000/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1000<F: Float>(t10975: F, t10978: F, t301: F, t761: F, t758: F, t10943: F, t5956: F, t5729: F, t2030: F, t3650: F, t2900: F, t302: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10979 = t10975 + t10978;
    let t10981 = t301 * t10979 * t761;
    let t10982 = t758 * t10981;
    let t10985 = t10943 * t5956;
    let t10986 = t758 * t10985;
    let t10989 = t10943 * t5729;
    let t10990 = t758 * t10989;
    let t10993 = t2030 * t3650;
    let t10994 = t2900 * t10993;
    let t10995 = t302 * t10994;
    (t10979, t10981, t10982, t10985, t10986, t10989, t10990, t10993, t10994, t10995)
}
