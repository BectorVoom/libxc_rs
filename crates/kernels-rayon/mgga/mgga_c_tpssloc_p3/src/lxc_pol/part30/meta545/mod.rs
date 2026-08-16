//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta545 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1896;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta545(t26231: f64, t26251: f64, t26255: f64, t26266: f64, t26361: f64, t26393: f64, t26406: f64, t26429: f64, t26127: f64, t19299: f64, t33: f64, t22505: f64, t22510: f64, t5392: f64, t5398: f64, t6500: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27012, t27019, t27022, t27027, t27067, t27082, t27088, t27096, t27166, t27937, t27948) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1896(t26231, t26251, t26255, t26266, t26361, t26393, t26406, t26429, t26127, t19299, t33, t22505, t22510, t5392, t5398, t6500);
    (t27012, t27019, t27022, t27027, t27067, t27082, t27088, t27096, t27166, t27937, t27948)
}
