//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta229 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1053;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta229(t3297: f64, t5971: f64, t136: f64, t1113: f64, t5975: f64, t5979: f64, t3282: f64, t3294: f64, t4721: f64, t4770: f64, t5973: f64, t5977: f64, t5981: f64, t5993: f64, t6000: f64, t6006: f64, t6008: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t6011, t6012, t6014, t6015, t6017, t6018, t6020) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1053(t3297, t5971, t136, t1113, t5975, t5979, t3282, t3294, t4721, t4770, t5973, t5977, t5981, t5993, t6000, t6006, t6008);
    (t6011, t6012, t6014, t6015, t6017, t6018, t6020)
}
