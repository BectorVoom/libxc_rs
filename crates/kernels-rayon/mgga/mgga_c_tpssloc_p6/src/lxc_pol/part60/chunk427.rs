//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 427/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk427(t1098: f64, t1657: f64, t1667: f64, t699: f64, t1128: f64, t1675: f64, t1147: f64, t1687: f64, t300: f64, t1171: f64, t1706: f64, t1420: f64, t972: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4740 = t1657 * t1098;
    let t4770 = t699 * t1667;
    let t4797 = t1675 * t1128;
    let t4835 = t1687 * t1147;
    let t4869 = t300 * t1687;
    let t4887 = t1706 * t1171;
    let t4889 = t1420 * t972;
    (t4740, t4770, t4797, t4835, t4869, t4887, t4889)
}
