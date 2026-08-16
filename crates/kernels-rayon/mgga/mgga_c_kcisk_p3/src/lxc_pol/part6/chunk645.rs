//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 645/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk645(t1899: f64, t8946: f64, t1873: f64, t1869: f64, t2473: f64, t6719: f64, t1799: f64, t1801: f64, t8518: f64, t1800: f64, t8510: f64, t5054: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8947 = t1899 * t8946;
    let t8948 = t1873 * t8947;
    let t8949 = t1869 * t8948;
    let t8951 = t6719 * t2473;
    let t8952 = t1799 * t8951;
    let t8954 = t1801 * t8518;
    let t8955 = t1800 * t8954;
    let t8956 = t1799 * t8955;
    let t8958 = t1801 * t8510;
    let t8959 = t1800 * t8958;
    let t8960 = t5054 * t8959;
    (t8947, t8948, t8949, t8951, t8952, t8954, t8955, t8956, t8958, t8959, t8960)
}
