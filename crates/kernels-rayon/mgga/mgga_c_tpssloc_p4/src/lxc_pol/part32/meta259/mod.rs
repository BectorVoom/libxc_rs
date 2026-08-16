//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta259 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1169;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta259(t1307: f64, t6968: f64, t6637: f64, t6888: f64, t2009: f64, t794: f64, t6897: f64, t1338: f64, t6604: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t6969, t6970, t6971, t6973, t6975, t6976) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1169(t1307, t6968, t6637, t6888, t2009, t794, t6897, t1338, t6604);
    (t6969, t6970, t6971, t6973, t6975, t6976)
}
