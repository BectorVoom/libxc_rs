//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta461 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1808;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1809;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1810;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta461(t23218: f64, t6553: f64, t1880: f64, t2553: f64, t6554: f64, t6552: f64, t218: f64, t23150: f64, t212: f64, t252: f64, t23171: f64, t23168: f64, t6556: f64, t22975: f64, t22979: f64, t23191: f64, t23198: f64, t23202: f64, t23207: f64, t23209: f64, t23211: f64, t23215: f64, t259: f64, t2597: f64, t2713: f64, t6632: f64, t6663: f64, t855: f64, t6547: f64, t6573: f64, t214: f64, t852: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23219, t23220, t23222, t23223, t23224, t23226, t23228) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1808(t23218, t6553, t1880, t2553, t6554, t6552, t218, t23150, t212, t252);
        let (t23229, t23231, t23232, t23234) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1809(t23228, t6554, t23171, t23168, t6556, t22975, t22979, t23191, t23198, t23202, t23207, t23209, t23211, t23215, t23220, t23224, t23226, t259, t2597, t2713, t6632, t6663, t855);
        let (t23235, t23236, t23237) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1810(t6547, t6573, t214, t852);
    (t23219, t23222, t23223, t23226, t23228, t23229, t23231, t23232, t23234, t23235, t23236, t23237)
}
