//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1085/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1085(t21812: f64, t21815: f64, t21829: f64, t21832: f64, t21835: f64, t21956: f64, t21958: f64, t21960: f64, t21963: f64, t22224: f64, t22226: f64, t11292: f64, t21906: f64) -> (f64, f64) {
    let t22227 = t21956 + t21958 + t21960 - t21963 + t21812 + t21815 + t21829 - t21832 + t21835 - t22224 - t22226;
    let t22228 = t11292 * t21906;
    (t22227, t22228)
}
