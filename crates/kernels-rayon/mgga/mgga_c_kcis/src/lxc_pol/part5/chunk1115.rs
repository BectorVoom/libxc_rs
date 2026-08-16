//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1115/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1115(t18787: f64, t934: f64, t4600: f64, t313: f64, t6338: f64, t1045: f64, t3293: f64, t1098: f64, t6590: f64, t6320: f64, t1670: f64, t4625: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18788 = t18787 * t934;
    let t18789 = t4600 * t18788;
    let t18792 = t313 * t6338;
    let t18793 = t18792 * t1045;
    let t18794 = t3293 * t18793;
    let t18800 = t1098 * t6590;
    let t18803 = t6320 * t934;
    let t18808 = t1670 * t4625;
    (t18788, t18789, t18793, t18794, t18800, t18803, t18808)
}
