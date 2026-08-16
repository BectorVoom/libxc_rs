//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta576 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2024;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta576(t12328: f64, t2003: f64, t12248: f64, t59: f64, t12267: f64, t6944: f64, t1336: f64, t2690: f64, t6943: f64, t1354: f64, t22770: f64, t22779: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t80900, t80901, t80910, t80914, t80915, t80920) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2024(t12328, t2003, t12248, t59, t12267, t6944, t1336, t2690, t6943, t1354, t22770, t22779);
    (t80900, t80901, t80910, t80914, t80915, t80920)
}
