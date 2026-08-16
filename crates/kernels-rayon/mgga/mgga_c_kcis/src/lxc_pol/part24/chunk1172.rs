//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1172/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1172(t3668: f64, t7807: f64, t10497: f64, t2183: f64, t11061: f64, t7788: f64, t7790: f64, t1071: f64, t3622: f64, t26954: f64, t27076: f64, t26996: f64, t993: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t92576 = t7807 * t3668;
    let t92581 = t2183 * t10497;
    let t92600 = t7788 * t11061 * t7790;
    let t92651 = t3622 * t1071;
    let t92657 = t27076 * t26954;
    let t92693 = t993 * t26996;
    (t92576, t92581, t92600, t92651, t92657, t92693)
}
