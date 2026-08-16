//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 297/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk297(t1002: f64, t1008: f64, t992: f64, t1014: f64, t1020: f64, t1024: f64, t1028: f64, t1041: f64, t1047: f64) -> (f64, f64) {
    let t1104 = 0.20855578275249024918e-2_f64 * t992 + 0.60736713313768998073e-4_f64 * t1002 - 0.43449121406768801913e-4_f64 * t1008;
    let t1112 = 0.27801896084645508334e-2_f64 * t1014 + 0.20241536458333333335e-4_f64 * t1020 - 0.17376185052903442709e-3_f64 * t1024 - 0.2318836277704281739e-4_f64 * t1028 - 0.16882592796244404291e-6_f64 * t1041 + 0.14492726735651760868e-5_f64 * t1047;
    (t1104, t1112)
}
