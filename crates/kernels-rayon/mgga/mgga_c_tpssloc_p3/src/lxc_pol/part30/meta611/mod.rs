//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta611 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2006;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2007;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta611(t23511: f64, t6733: f64, t1049: f64, t6743: f64, t883: f64, t221: f64, t697: f64, t1926: f64, t6790: f64, t6787: f64, t23631: f64, t974: f64, t976: f64, t984: f64, t1009: f64, t343: f64, t25490: f64, t210: f64, t23632: f64, t23668: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t82620, t82625, t82632) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2006(t23511, t6733, t1049, t6743, t883, t221, t697, t1926);
        let (t82633, t82635, t82653, t82654, t82655, t82668) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2007(t6790, t82632, t6787, t23631, t974, t976, t984, t1009, t343, t25490, t210, t23632, t23668);
    (t82620, t82625, t82632, t82633, t82635, t82653, t82654, t82655, t82668)
}
