//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta720 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2287;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2288;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2289;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta720(t55353: f64, t7769: f64, t16521: f64, t7467: f64, t1873: f64, t19534: f64, t3941: f64, t28017: f64, t671: f64, t20173: f64, t28899: f64, t1395: f64, t5456: f64, t20162: f64, t6534: f64, t26545: f64, t33185: f64, t12524: f64, t28896: f64, t5493: f64, t2174: f64, t6470: f64, t25: f64, t265: f64, t394: f64, t100624: f64, t1409: f64, t16558: f64, t2116: f64, t27373: f64, t29507: f64, t3966: f64, t40: f64, t5398: f64, t607: f64, t7274: f64, t7992: f64, t99069: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t100917, t100921, t100924, t100927, t100929, t100930) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2287(t55353, t7769, t16521, t7467, t1873, t19534, t3941, t28017, t671, t20173, t28899, t1395, t5456);
        let (t100932, t100934, t100936, t100938, t100941, t103103) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2288(t100930, t1873, t20162, t6534, t26545, t33185, t12524, t28896, t3941, t5493, t2174, t6470);
        let t103125 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2289(t25, t265, t394, t100624, t1409, t16558, t2116, t27373, t29507, t3966, t40, t5398, t607, t7274, t7992, t99069, dens_threshold, rho0, zeta_threshold);
    (t100917, t100921, t100924, t100927, t100929, t100932, t100934, t100936, t100938, t100941, t103103, t103125)
}
