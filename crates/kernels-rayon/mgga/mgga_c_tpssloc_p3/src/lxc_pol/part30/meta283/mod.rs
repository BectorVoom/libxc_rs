//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta283 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1279;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1280;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1281;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta283(t533: f64, t7752: f64, t1390: f64, t1983: f64, t2019: f64, t5161: f64, t113: f64, t1442: f64, t1459: f64, t1774: f64, t1849: f64, t1869: f64, t1976: f64, t1980: f64, t510: f64, t574: f64, t6517: f64, t652: f64, t7451: f64, t7457: f64, t7460: f64, t7463: f64, t7470: f64, t7472: f64, t7670: f64, t7681: f64, t7686: f64, t7690: f64, t3: f64, t1873: f64, t5371: f64, t1458: f64, t3941: f64, t1401: f64, t7467: f64, t577: f64, t7010: f64, t1714: f64, t460: f64, t2018: f64, t3701: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7753, t7754, t7756, t7758) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1279(t533, t7752, t1390, t1983, t2019, t5161, t113, t1442, t1459, t1774, t1849, t1869, t1976, t1980, t510, t574, t6517, t652, t7451, t7457, t7460, t7463, t7470, t7472, t7670, t7681, t7686, t7690);
        let (t7759, t7768, t7769) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1280(t3, t7758, t1873, t5371, t1458);
        let (t7774, t8034, t8643) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1281(t3941, t7769, t1401, t7467, t1458, t577, t7010, t7758, t7768, t1714, t460, t2018, t3701);
    (t7753, t7754, t7756, t7758, t7759, t7769, t7774, t8034, t8643)
}
