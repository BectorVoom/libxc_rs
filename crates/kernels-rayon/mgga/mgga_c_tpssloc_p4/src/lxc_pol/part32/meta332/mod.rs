//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta332 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1364;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta332(t3749: f64, t9577: f64, t1314: f64, t2566: f64, t3741: f64, t3732: f64, t792: f64, t782: f64, t1365: f64, t154: f64, t205: f64, t116: f64, t547: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12196, t12199, t12200, t12202, t12211, t12214, t12215, t12225) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1364(t3749, t9577, t1314, t2566, t3741, t3732, t792, t782, t1365, t154, t205, t116, t547);
    (t12196, t12199, t12200, t12202, t12211, t12214, t12215, t12225)
}
