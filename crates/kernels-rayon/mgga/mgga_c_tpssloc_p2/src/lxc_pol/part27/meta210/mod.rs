//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta210 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1052;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1053;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1054;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta210(t1409: f64, t3242: f64, t607: f64, t3240: f64, t123: f64, t3247: f64, t1088: f64, t1089: f64, t3966: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4723, t4724) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1052(t1409, t3242, t607);
        let (t4725, t4726, t4728, t4729) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1053(t3240, t4724, t123, t1409, t3247, t607);
        let (t4730, t4731, t4733) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1054(t1088, t4729, t123, t1089, t3966);
    (t4723, t4724, t4725, t4726, t4728, t4729, t4730, t4731, t4733)
}
