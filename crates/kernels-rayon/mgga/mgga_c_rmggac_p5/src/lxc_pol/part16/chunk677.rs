//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 677/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk677(t1763: f64, t7577: f64, t739: f64, t2289: f64, t2412: f64, t1942: f64, t1986: f64, t675: f64, t2310: f64, t1835: f64, t202: f64, t461: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9812 = t7577 * t1763;
    let t9813 = t739 * t9812;
    let t9815 = t2412 * t2289;
    let t9817 = t1986 * t1942;
    let t9818 = t675 * t9817;
    let t9820 = t2412 * t2310;
    let t9824 = t1835 * t202;
    let t9825 = t9824 * t461;
    (t9812, t9813, t9815, t9817, t9818, t9820, t9824, t9825)
}
