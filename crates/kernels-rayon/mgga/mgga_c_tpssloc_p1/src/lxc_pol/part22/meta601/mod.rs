//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta601 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2123;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta601(t10523: f64, t1573: f64, t10629: f64, t48096: f64, t47730: f64, t48155: f64, t1556: f64, t2842: f64, t10828: f64, t1580: f64, t2841: f64, t4351: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49099, t49104, t49139, t49144, t49200, t49226, t49263, t49269) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2123(t10523, t1573, t10629, t48096, t47730, t48155, t1556, t2842, t10828, t1580, t2841, t4351);
    (t49099, t49104, t49139, t49144, t49200, t49226, t49263, t49269)
}
