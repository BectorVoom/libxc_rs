//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1062/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1062(t2297: f64, t4256: f64, t7450: f64, t839: f64, t1131: f64, t2030: f64, t4262: f64, t7447: f64, t8924: f64, t7440: f64, t8929: f64, t2282: f64, t7600: f64) -> (f64, f64, f64, f64, f64) {
    let t34887 = t7450 * t4256 * t2297 * t839;
    let t34891 = t2030 * t4262 * t2297 * t1131;
    let t34893 = t7447 * t8924;
    let t34895 = t7440 * t8929;
    let t34897 = t7600 * t2282;
    (t34887, t34891, t34893, t34895, t34897)
}
