//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1260/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1260(t1143: f64, t3694: f64, t10942: f64, t306: f64, t1123: f64, t3669: f64, t1133: f64, t3638: f64, t3650: f64, t2036: f64, t10979: f64, t5955: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t30843 = t1143 * t3694;
    let t30868 = t306 * t10942;
    let t30885 = t3669 * t1123;
    let t30893 = t1133 * t3638;
    let t30897 = t1133 * t3650;
    let t30898 = t2036 * t30897;
    let t30910 = t306 * t10979;
    let t30916 = t5955 * t3650;
    (t30843, t30868, t30885, t30893, t30897, t30898, t30910, t30916)
}
