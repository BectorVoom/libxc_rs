//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1601/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1601(t13487: f64, t22960: f64, t606: f64, t776: f64, t25: f64, t2553: f64, t1887: f64, t6581: f64) -> (f64, f64, f64, f64) {
    let t22961 = t22960 * t13487;
    let t22964 = t606 * t776;
    let t22968 = t25 * t2553;
    let t22986 = t6581 * t1887;
    (t22961, t22964, t22968, t22986)
}
