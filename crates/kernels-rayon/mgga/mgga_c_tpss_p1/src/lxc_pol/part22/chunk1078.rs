//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1078/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1078(t2885: f64, t4079: f64, t1027: f64, t2877: f64, t4087: f64, t1985: f64, t4046: f64, t1038: f64, t141: f64, t4048: f64, t664: f64) -> (f64, f64, f64, f64, f64) {
    let t11864 = t2885 * t4079;
    let t11865 = t11864 * t1027;
    let t11867 = t4087 * t2877;
    let t11869 = t4046 * t1985;
    let t11870 = t1038 * t11869;
    let t11871 = t141 * t11870;
    let t11873 = t664 * t4048;
    (t11865, t11867, t11869, t11871, t11873)
}
