//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 671/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk671(t4978: f64, t676: f64, t1798: f64, t618: f64, t144: f64, t1975: f64, t1453: f64, t190: f64, t1303: f64, t1672: f64, t1839: f64, t674: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4979 = t676 * t4978;
    let t4991 = t618 * t1798;
    let t4995 = t1975 * t144;
    let t5011 = t190 * t1453;
    let t5017 = t1672 * t1303;
    let t5021 = t674 * t1839;
    (t4979, t4991, t4995, t5011, t5017, t5021)
}
