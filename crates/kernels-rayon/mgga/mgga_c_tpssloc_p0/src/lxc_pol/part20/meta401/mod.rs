//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta401 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1797;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1798;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta401(t13748: f64, t973: f64, t1611: f64, t3088: f64, t1036: f64, t4617: f64, t1023: f64, t4347: f64, t3071: f64, t10422: f64, t4574: f64, t3070: f64, t1597: f64, t4509: f64, t10237: f64, t10189: f64, t344: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13750, t13751, t13758, t13761, t13762, t13765, t13767) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1797(t13748, t973, t1611, t3088, t1036, t4617, t1023, t4347, t3071, t10422, t4574, t3070);
        let (t13769, t13770, t13779) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1798(t1597, t4509, t10237, t10189, t344);
    (t13750, t13751, t13758, t13761, t13762, t13765, t13767, t13769, t13770, t13779)
}
