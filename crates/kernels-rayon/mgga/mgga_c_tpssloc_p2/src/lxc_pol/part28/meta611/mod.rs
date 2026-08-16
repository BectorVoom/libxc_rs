//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta611 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1923;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1924;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta611(t22633: f64, t26421: f64, t3856: f64, t6976: f64, t26462: f64, t6914: f64, t22705: f64, t26414: f64, t81228: f64, t26415: f64, t81159: f64, t3851: f64, t26418: f64, t7736: f64, t80854: f64, t81064: f64, t22704: f64, t26410: f64, t26432: f64, t6897: f64, t794: f64, t22642: f64, t22690: f64, t26395: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90933, t90956, t90961, t90963, t90968) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1923(t22633, t26421, t3856, t6976, t26462, t6914, t22705, t26414, t81228, t26415, t81159, t3851);
        let (t90970, t90980, t90983, t90987, t90993) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1924(t26418, t6914, t7736, t80854, t81064, t22704, t22705, t26410, t26432, t6897, t794, t22642, t22690, t26395);
    (t90933, t90956, t90961, t90963, t90968, t90970, t90980, t90983, t90987, t90993)
}
