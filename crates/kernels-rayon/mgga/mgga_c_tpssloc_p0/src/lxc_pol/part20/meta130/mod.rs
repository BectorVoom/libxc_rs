//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta130 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk851;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta130(t135: f64, t999: f64, t973: f64, t2250: f64, t998: f64, t974: f64, t2770: f64, t2978: f64, t2244: f64, t2775: f64, t976: f64, t1005: f64, t1036: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3139, t3140, t3142, t3143, t3147, t3148, t3152, t3153, t3156) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk851(t135, t999, t973, t2250, t998, t974, t2770, t2978, t2244, t2775, t976, t1005, t1036);
    (t3139, t3140, t3142, t3143, t3147, t3148, t3152, t3153, t3156)
}
