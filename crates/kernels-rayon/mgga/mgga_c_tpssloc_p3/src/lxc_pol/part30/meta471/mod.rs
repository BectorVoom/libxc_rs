//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta471 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1760;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1761;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1762;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1763;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta471(t3030: f64, t344: f64, t1014: f64, t1011: f64, t360: f64, t225: f64, t6733: f64, t1949: f64, t2966: f64, t1920: f64, t6680: f64, t6781: f64, t6805: f64, t968: f64, t210: f64, t6795: f64, t6688: f64, t974: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t23602 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1760(t3030, t344);
        let (t23603, t23604, t23613) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1761(t1014, t23602, t1011, t360, t225, t6733);
        let (t23617, t23619, t23626, t23629, t23631) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1762(t1949, t2966, t1920, t6680, t6781, t6805, t968, t210, t6795);
        let (t23632, t23633) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1763(t6688, t974, t23631);
    (t23602, t23603, t23604, t23613, t23617, t23619, t23626, t23629, t23631, t23632, t23633)
}
