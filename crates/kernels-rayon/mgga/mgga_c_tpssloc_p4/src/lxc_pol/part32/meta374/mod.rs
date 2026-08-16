//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta374 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1427;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1428;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta374(t1788: f64, t2225: f64, t2221: f64, t225: f64, t5213: f64, t5211: f64, t1372: f64, t1824: f64, t5286: f64, t562: f64, t12248: f64, t68: f64, t544: f64, t5230: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15982, t15984, t16022, t16030, t16036, t16040, t16046) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1427(t1788, t2225, t2221, t225, t5213, t5211, t1372, t1824, t5286, t562, t12248, t68);
        let (t16047, t16060) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1428(t16046, t544, t5230, t68);
    (t15982, t15984, t16022, t16030, t16036, t16040, t16047, t16060)
}
