//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta112 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk677;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk678;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk679;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk680;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta112(t154: f64, t2559: f64, t222: f64, t2563: f64, t805: f64, t68: f64, t808: f64, t816: f64, t809: f64, t838: f64, t842: f64, t233: f64, t813: f64, t236: f64, t240: f64, t812: f64, t232: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2600, t2602, t2603, t2617) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk677(t154, t2559, t222, t2563, t805, t68, t808);
        let (t2618, t2621, t2623, t2627) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk678(t2617, t816, t809, t838, t842, t233, t813);
        let t2628 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk679(t236, t2627);
        let (t2629, t2630, t2632) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk680(t240, t2628, t812, t232);
    (t2600, t2602, t2603, t2617, t2618, t2621, t2623, t2627, t2628, t2629, t2630, t2632)
}
