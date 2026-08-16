//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta279 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1260;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1261;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1262;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta279(t533: f64, t7752: f64, t1390: f64, t1983: f64, t2019: f64, t5161: f64, t1873: f64, t5371: f64, t1458: f64, t3941: f64, t1401: f64, t7467: f64, t1409: f64, t1419: f64, t56: f64, t6503: f64, t7251: f64, t67: f64, t1864: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7753, t7754, t7755, t7756, t7757, t7768, t7769) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1260(t533, t7752, t1390, t1983, t2019, t5161, t1873, t5371, t1458);
        let (t7771, t7773, t7973, t7974) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1261(t3941, t7769, t1401, t7467, t1409, t1419, t56, t6503, t7251, t67);
        let t7975 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1262(t1864, t7974);
    (t7753, t7754, t7755, t7756, t7757, t7768, t7769, t7771, t7773, t7973, t7974, t7975)
}
