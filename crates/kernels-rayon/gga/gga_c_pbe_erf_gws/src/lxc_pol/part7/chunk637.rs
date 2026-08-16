//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 637/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk637(t173: f64, t4980: f64, t184: f64, t199: f64, t1783: f64, t636: f64, t1841: f64, t735: f64, t1648: f64, t1898: f64, t155: f64, t589: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4981 = t173 * t4980;
    let t4982 = t4981 * t184;
    let t4984 = 2.0_f64 / 15.0_f64 * t4982 * t199;
    let t4985 = t1783 * t636;
    let t4986 = 8.0_f64 / 15.0_f64 * t4985;
    let t4987 = t1841 * t735;
    let t4990 = 8.0_f64 / 15.0_f64 * t1648 * t1898;
    let t4991 = t155 * t589;
    (t4981, t4982, t4984, t4986, t4987, t4990, t4991)
}
