//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta273 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1306;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1307;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1308;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta273<F: Float>(t533: F, t7752: F, t1390: F, t1983: F, t2019: F, t5161: F, t113: F, t1442: F, t1459: F, t1774: F, t1849: F, t1869: F, t1976: F, t1980: F, t510: F, t574: F, t6517: F, t652: F, t7451: F, t7457: F, t7460: F, t7463: F, t7470: F, t7472: F, t7670: F, t7681: F, t7686: F, t7690: F, t3: F, t1873: F, t5371: F, t1458: F, t3941: F, t1401: F, t7467: F, t577: F, t7010: F, t2018: F, t3701: F) -> (F, F, F, F, F, F, F, F) {
        let (t7753, t7754, t7756, t7758) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1306::<F>(t533, t7752, t1390, t1983, t2019, t5161, t113, t1442, t1459, t1774, t1849, t1869, t1976, t1980, t510, t574, t6517, t652, t7451, t7457, t7460, t7463, t7470, t7472, t7670, t7681, t7686, t7690);
        let (t7759, t7768, t7769) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1307::<F>(t3, t7758, t1873, t5371, t1458);
        let (t7774, t8643) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1308::<F>(t3941, t7769, t1401, t7467, t1458, t577, t7010, t7758, t7768, t2018, t3701);
    (t7753, t7754, t7756, t7758, t7759, t7769, t7774, t8643)
}
