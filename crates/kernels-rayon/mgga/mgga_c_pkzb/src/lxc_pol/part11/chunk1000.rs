//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1000/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1000(t10975: f64, t10978: f64, t301: f64, t761: f64, t758: f64, t10943: f64, t5956: f64, t5729: f64, t2030: f64, t3650: f64, t2900: f64, t302: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
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
