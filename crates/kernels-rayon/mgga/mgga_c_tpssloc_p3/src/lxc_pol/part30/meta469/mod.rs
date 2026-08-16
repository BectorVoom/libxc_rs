//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta469 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1756;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1757;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta469(t3037: f64, t6753: f64, t3033: f64, t1004: f64, t6764: f64, t1036: f64, t6750: f64, t6759: f64, t3: f64, t6740: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t23540, t23541) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1756(t3037, t6753, t3033);
        let (t23544, t23554, t23560, t23562) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1757(t1004, t6764, t1036, t6750, t6759, t3, t6740);
    (t23540, t23541, t23544, t23554, t23560, t23562)
}
