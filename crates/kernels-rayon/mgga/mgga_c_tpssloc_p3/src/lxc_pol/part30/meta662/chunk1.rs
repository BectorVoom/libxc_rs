//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2085/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2085(t22704: f64, t5336: f64, t80798: f64, t22724: f64, t26436: f64, t26423: f64, t81159: f64, t215: f64, t22839: f64, t562: f64, t80854: f64, t1338: f64, t26328: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t90898 = t22704 * t80798 * t5336;
    let t90899 = 0.16449340668482264365e-1_f64 * t90898;
    let t90900 = t22724 * t26436;
    let t90912 = t81159 * t26423;
    let t90913 = 0.76763589786250567036e-1_f64 * t90912;
    let t90914 = t22839 * t215;
    let t90915 = t80854 * t562;
    let t90952 = t1338 * t26328;
    (t90899, t90900, t90913, t90914, t90915, t90952)
}
