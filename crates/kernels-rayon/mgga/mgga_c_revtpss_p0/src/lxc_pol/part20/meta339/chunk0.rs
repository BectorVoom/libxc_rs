//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1264/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1264(t15687: f64, t3088: f64, t3317: f64, t12131: f64, t3095: f64, t1087: f64, t11773: f64, t372: f64, t4801: f64, t1062: f64, t11940: f64, t11788: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15688 = t3088 * t15687;
    let t15689 = t3317 * t15688;
    let t15692 = t12131 * t3095;
    let t15700 = t1087 * t11773;
    let t15701 = t372 * t4801;
    let t15716 = t11940 * t1062;
    let t15725 = t11788 * t1062;
    (t15688, t15689, t15692, t15700, t15701, t15716, t15725)
}
