//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1174/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1174(t11000: f64, t1268: f64, t1241: f64, t209: f64, t7787: f64, t1094: f64, t283: f64, t1130: f64, t46978: f64, t7788: f64, t7795: f64, t92748: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t92787 = t11000 * t1268;
    let t92794 = t1241 * t209;
    let t92795 = t7787 * t92794;
    let t92807 = t1094 * t283;
    let t92808 = t92807 * t1130;
    let t92896 = t7788 * t46978 * t7795;
    let t92898 = t7788 * t92748;
    (t92787, t92794, t92795, t92807, t92808, t92896, t92898)
}
