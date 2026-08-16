//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta484 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1825;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1826;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1827;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta484(t461: f64, t52: f64, t1009: f64, t7324: f64, t1210: f64, t7330: f64, t3502: f64, t3504: f64, t3500: f64, sigma2: f64, t7337: f64, t1202: f64, t7344: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24719, t24720, t24721, t24722, t24723, t24727, t24728, t24729) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1825(t461, t52, t1009, t7324, t1210, t7330, t3502, t3504, t3500, sigma2);
        let (t24732, t24733) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1826(t3504, t7337, t3500);
        let t24736 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1827(t1202, t7344);
    (t24719, t24720, t24721, t24722, t24723, t24727, t24728, t24729, t24732, t24733, t24736)
}
