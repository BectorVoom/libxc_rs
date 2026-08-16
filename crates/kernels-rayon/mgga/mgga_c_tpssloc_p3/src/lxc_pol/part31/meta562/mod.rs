//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta562 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1791;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1792;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta562(t81575: f64, t25251: f64, t87049: f64, t23012: f64, t7529: f64, t23110: f64, t23185: f64, t25241: f64, t1484: f64, t852: f64, t81595: f64, t81602: f64, t252: f64, t4119: f64, t22690: f64, t7520: f64, t81573: f64, t25324: f64, t6562: f64, t794: f64, t23030: f64, t25258: f64, t22893: f64, t23164: f64, t25306: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87073, t87078, t87080, t87100, t87111, t87119, t87127) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1791(t81575, t25251, t87049, t23012, t7529, t23110, t23185, t25241, t1484, t852, t81595, t81602);
        let (t87130, t87140, t87153, t87155, t87165) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1792(t252, t4119, t22690, t7520, t81573, t25324, t6562, t794, t23030, t25258, t22893, t23164, t25306);
    (t87073, t87078, t87080, t87100, t87111, t87119, t87127, t87130, t87140, t87153, t87155, t87165)
}
