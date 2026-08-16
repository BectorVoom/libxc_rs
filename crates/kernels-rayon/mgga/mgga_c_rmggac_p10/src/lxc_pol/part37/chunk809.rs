//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 809/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk809(t2841: f64, t498: f64, t14236: f64, t2067: f64, t69588: f64, t13848: f64, t13850: f64, t8608: f64, t13858: f64, t2412: f64, t15220: f64, t2191: f64) -> (f64, f64, f64, f64) {
    let t74571 = t2841 * t498;
    let t74574 = t14236 * t69588 * t2067 * t74571;
    let t74577 = t8608 * t13848 * t13850;
    let t74579 = t2412 * t13858;
    let t74581 = t2191 * t15220;
    (t74574, t74577, t74579, t74581)
}
