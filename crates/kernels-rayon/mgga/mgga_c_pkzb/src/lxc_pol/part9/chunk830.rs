//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 830/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk830(t2107: f64, t5974: f64, t2104: f64, t1843: f64, t759: f64, t761: f64, t2105: f64, t178: f64, t2094: f64, t752: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5975 = t5974 * t2107;
    let t5976 = t2104 * t5975;
    let t5978 = t1843 * t759;
    let t5979 = t5978 * t761;
    let t5980 = t2105 * t5979;
    let t5983 = t2094 * t178;
    let t5984 = t752 * t5983;
    (t5975, t5976, t5978, t5979, t5980, t5984)
}
