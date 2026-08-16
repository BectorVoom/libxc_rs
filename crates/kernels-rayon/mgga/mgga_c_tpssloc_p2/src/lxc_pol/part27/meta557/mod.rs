//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta557 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2000;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta557(t85: f64, t24: f64, t12019: f64, t566: f64, t3700: f64, t2751: f64, t10108: f64, t257: f64, t10163: f64, t386: f64, t3215: f64, t111: f64, t3931: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39063, t40590, t40611, t40772, t40889, t43603, t43637, t45560) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2000(t85, t24, t12019, t566, t3700, t2751, t10108, t257, t10163, t386, t3215, t111, t3931);
    (t39063, t40590, t40611, t40772, t40889, t43603, t43637, t45560)
}
