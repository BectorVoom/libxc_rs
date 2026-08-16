//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta749 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2538;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta749(t2873: f64, t4587: f64, t11298: f64, t1596: f64, t11466: f64, t1633: f64, t11299: f64, t1609: f64, t51913: f64, t51915: f64, t51973: f64, t52035: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t52505, t52508, t52511, t52514, t52546, t52547, t52573, t52597) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2538(t2873, t4587, t11298, t1596, t11466, t1633, t11299, t1609, t51913, t51915, t51973, t52035);
    (t52505, t52508, t52511, t52514, t52546, t52547, t52573, t52597)
}
