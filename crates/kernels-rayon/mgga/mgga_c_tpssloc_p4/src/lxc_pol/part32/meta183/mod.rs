//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta183 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk895;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk896;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk897;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta183(t3449: f64, t4904: f64, t3448: f64, t461: f64, t4729: f64, t1178: f64, t3966: f64, t1177: f64, t135: f64, t1716: f64, t1174: f64, t1714: f64, t3451: f64, t3295: f64, t3464: f64, t4770: f64, t4773: f64, t4776: f64, t4779: f64, t457: f64, t460: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4905, t4908, t4909, t4912, t4913, t4916, t4917, t4919) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk895(t3449, t4904, t3448, t461, t4729, t1178, t3966, t1177, t135, t1716, t1174, t1714);
        let (t4920, t4928) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk896(t3451, t4919, t3295, t3464, t4770, t4773, t4776, t4779);
        let t4930 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk897(t457, t4928, t460);
    (t4905, t4908, t4909, t4912, t4913, t4916, t4917, t4919, t4920, t4928, t4930)
}
