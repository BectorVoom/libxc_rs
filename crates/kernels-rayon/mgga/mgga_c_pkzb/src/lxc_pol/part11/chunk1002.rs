//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1002/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1002(t10767: f64, t154: f64, t742: f64, t10932: f64, t5656: f64, t287: f64, t3542: f64, t1137: f64, t5693: f64, t3645: f64, t3679: f64, t2105: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11011 = t154 * t742 * t10767;
    let t11015 = t154 * t5656 * t10932;
    let t11019 = t287 * t3542;
    let t11020 = t1137 * t11019;
    let t11021 = t5693 * t11020;
    let t11024 = t3679 * t3645;
    let t11025 = t2105 * t11024;
    (t11011, t11015, t11019, t11020, t11021, t11024, t11025)
}
