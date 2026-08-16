//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta667 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2099;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta667(t24847: f64, t64825: f64, t974: f64, t8067: f64, t85660: f64, t11715: f64, t491: f64, t85964: f64, t27488: f64, t8070: f64, t225: f64, t27654: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t94963, t94966, t95000, t95005, t95033, t95035) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2099(t24847, t64825, t974, t8067, t85660, t11715, t491, t85964, t27488, t8070, t225, t27654);
    (t94963, t94966, t95000, t95005, t95033, t95035)
}
