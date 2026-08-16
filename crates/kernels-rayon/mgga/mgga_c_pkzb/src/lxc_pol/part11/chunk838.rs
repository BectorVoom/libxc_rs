//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 838/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk838(t3418: f64, t5257: f64, t1721: f64, t3441: f64, t600: f64, t179: f64, t3410: f64, t5391: f64, t8914: f64, t164: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8946 = t5257 * t3418;
    let t8948 = t3441 * t1721;
    let t8949 = t8948 * t600;
    let t8950 = t179 * t8949;
    let t8953 = t3410 * t5391;
    let t8954 = t8953 * t600;
    let t8955 = t179 * t8954;
    let t8958 = t8914 * t600;
    let t8959 = t179 * t8958;
    let t8962 = t3410 * t164;
    (t8946, t8948, t8949, t8950, t8953, t8954, t8955, t8958, t8959, t8962)
}
