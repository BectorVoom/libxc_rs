//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 815/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk815(t13862: f64, t14041: f64, t8615: f64, t14078: f64, t8659: f64, t14125: f64, t236: f64, t68884: f64, t8602: f64, t495: f64, t598: f64, t68876: f64) -> (f64, f64, f64, f64) {
    let t74655 = t14041 * t13862 * t8615;
    let t74657 = t8659 * t14078;
    let t74662 = t68884 * t14125 * t236 * t8602;
    let t74667 = t68876 * t14125 * t236 * t598 * t495;
    (t74655, t74657, t74662, t74667)
}
