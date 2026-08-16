//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 778/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk778(t1174: f64, t15363: f64, t1420: f64, t1887: f64, t337: f64, t1714: f64, t4899: f64, t15026: f64, t3032: f64, t3514: f64, t1742: f64, t3036: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15364 = t1174 * t15363;
    let t15376 = t1420 * t337 * t1887;
    let t15390 = t4899 * t1714;
    let t15437 = t15026 * t3032;
    let t15438 = t15437 * t3514;
    let t15501 = t1742 * t3036;
    (t15364, t15376, t15390, t15437, t15438, t15501)
}
