//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta530 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2001;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2002;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2003;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta530(t85: f64, t24: f64, t10276: f64, t73: f64, t11152: f64, t76: f64, t41: f64, t42: f64, t53: f64, t54: f64, t9576: f64, t2405: f64, t2420: f64, t702: f64, t2412: f64, t125: f64, t2409: f64, t2418: f64, t9481: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39063, t39096, t39114, t39159, t39168, t39210, t39246) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2001(t85, t24, t10276, t73, t11152, t76, t41, t42, t53, t54, t9576, t2405);
        let t39249 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2002(t2420, t39246, t702);
        let (t39253, t39256) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2003(t2412, t125, t2409, t2418, t9481);
    (t39063, t39096, t39114, t39159, t39168, t39210, t39246, t39249, t39253, t39256)
}
