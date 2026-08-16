//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta713 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2242;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta713(t16924: f64, t23146: f64, t16914: f64, t16903: f64, t5593: f64, t81749: f64, t16845: f64, t25084: f64, t16893: f64, t17017: f64, t16841: f64, t87368: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98612, t98614, t98616, t98618, t98620, t98622, t98624, t98626) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2242(t16924, t23146, t16914, t16903, t5593, t81749, t16845, t25084, t16893, t17017, t16841, t87368);
    (t98612, t98614, t98616, t98618, t98620, t98622, t98624, t98626)
}
