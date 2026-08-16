//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 851/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk851(t2242: f64, t6143: f64, t6142: f64, t339: f64, t346: f64, t2204: f64, t836: f64, t2203: f64, t2209: f64, t6087: f64, t6090: f64, t6093: f64, t6108: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6144 = t6143 * t2242;
    let t6146 = 0.96491876992155210402e2_f64 * t6142 * t6144;
    let t6149 = 1.0_f64 / t339 / t346 / 4.0_f64;
    let t6150 = t2204 * t836;
    let t6151 = t6149 * t6150;
    let t6153 = t2203 * t836;
    let t6154 = t6153 * t2209;
    let t6156 = 28.0_f64 / 27.0_f64 * t6087;
    let t6158 = -t6156 + 4.0_f64 / 3.0_f64 * t6090 - t6093 + t6108;
    (t6144, t6146, t6149, t6150, t6151, t6154, t6156, t6158)
}
