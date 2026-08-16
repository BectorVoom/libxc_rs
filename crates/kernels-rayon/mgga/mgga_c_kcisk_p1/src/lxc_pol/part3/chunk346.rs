//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 346/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk346(t1248: f64, t1636: f64, t1720: f64, t1699: f64, t1710: f64, t1712: f64, t1715: f64, t1719: f64, t620: f64) -> (f64, f64, f64) {
    let t1722 = t1248 * t1720 * t1636;
    let t1724 = 0.1898925e1_f64 * t1710 - t1712 - 0.29896666666666666667e0_f64 * t1699 + 0.3071625e0_f64 * t1715 - t1719 - 0.16431333333333333333e0_f64 * t1722;
    let t1725 = 1.0_f64 / t620;
    (t1722, t1724, t1725)
}
