//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta489 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1836;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1837;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta489(t24813: f64, t3502: f64, t1011: f64, t3508: f64, t3611: f64, t1209: f64, t475: f64, t1193: f64, t7372: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24814, t24815) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1836(t24813, t3502, t1011, t3508);
        let (t24816, t24817, t24820, t24821, t24822, t24823, t24826) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1837(t24815, t3611, t24814, t1209, t24813, t1011, t475, t1193, t7372);
    (t24814, t24815, t24816, t24817, t24820, t24821, t24822, t24823, t24826)
}
