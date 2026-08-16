//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta612 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2008;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2009;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta612(t23518: f64, t6733: f64, t23669: f64, t995: f64, t3158: f64, t6796: f64, t6802: f64, t23600: f64, t10336: f64, t1920: f64, t1949: f64, t2966: f64, t6805: f64, t135: f64, t23631: f64, t6688: f64, t23617: f64, t6680: f64, t10889: f64, t3033: f64, t6753: f64, t10510: f64, t6755: f64, t10870: f64, t6765: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t82683, t82713, t82716, t82717, t82736, t82799, t82809) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2008(t23518, t6733, t23669, t995, t3158, t6796, t6802, t23600, t10336, t1920, t1949, t2966, t6805);
        let (t82822, t82830, t82848, t82851, t82875) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2009(t135, t23631, t6688, t23617, t6680, t10889, t3033, t6753, t10510, t6755, t10870, t6765);
    (t82683, t82713, t82716, t82717, t82736, t82799, t82809, t82822, t82830, t82848, t82851, t82875)
}
