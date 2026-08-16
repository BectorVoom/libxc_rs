//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta605 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2076;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2077;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta605(t6790: f64, t82632: f64, t6787: f64, t225: f64, t23547: f64, t23631: f64, t974: f64, t976: f64, t984: f64, t1009: f64, t343: f64, t25490: f64, t6746: f64, t884: f64, t23384: f64, t23715: f64, t210: f64, t23632: f64, t23668: f64, t23628: f64, t6680: f64, t23669: f64, t995: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t82633, t82635, t82643, t82653, t82654, t82655) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2076(t6790, t82632, t6787, t225, t23547, t23631, t974, t976, t984, t1009, t343, t25490);
        let (t82657, t82661, t82668, t82694, t82713) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2077(t6746, t82655, t884, t23384, t23715, t210, t23632, t23668, t23628, t6680, t23669, t995);
    (t82633, t82635, t82643, t82653, t82654, t82655, t82657, t82661, t82668, t82694, t82713)
}
