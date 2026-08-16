//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta522 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1897;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1898;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta522(t26066: f64, t72: f64, t1410: f64, t2235: f64, t3961: f64, t605: f64, t3967: f64, t1433: f64, t645: f64, t12725: f64, t1873: f64, t19456: f64, t4028: f64, t6534: f64, t1458: f64, t649: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26067, t26070, t26073, t26076, t26090, t26109, t26111) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1897(t26066, t72, t1410, t2235, t3961, t605, t3967, t1433, t645, t12725, t1873, t19456);
        let (t26113, t26114) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1898(t4028, t6534, t1458, t649);
    (t26067, t26070, t26073, t26076, t26090, t26109, t26111, t26113, t26114)
}
