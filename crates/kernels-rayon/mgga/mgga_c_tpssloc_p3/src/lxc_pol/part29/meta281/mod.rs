//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta281 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1296;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1297;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1298;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1299;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta281(t28: f64, t265: f64, t504: f64, t1256: f64, t1763: f64, t193: f64, t336: f64, t4700: f64, t7398: f64, t7642: f64, t8090: f64, t1409: f64, t2161: f64, t52: f64, t7663: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t7997: f64, t1458: f64, t7266: f64, t7675: f64, t7678: f64, t7680: f64, t7983: f64, t113: f64, t1442: f64, t1459: f64, t1774: f64, t1849: f64, t2114: f64, t2165: f64, t2167: f64, t510: f64, t574: f64, t652: f64, t7457: f64, t7460: f64, t7463: f64, t7470: f64, t7686: f64, t7690: f64, t7755: f64, t7757: f64, t7989: f64, t3: f64, t577: f64, t7423: f64, t7768: f64, t7771: f64, t7773: f64, t2018: f64, t3701: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t8097, t8102) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1296(t28, t265, t504, t1256, t1763, t193, t336, t4700, t7398, t7642, t8090, t1409, t2161, t52, t7663, dens_threshold, rho1, zeta_threshold);
        let t8103 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1297(t7997, t8102);
        let (t8107, t8110) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1298(t1458, t7266, t7675, t7678, t7680, t7983, t113, t1442, t1459, t1774, t1849, t2114, t2165, t2167, t510, t574, t652, t7457, t7460, t7463, t7470, t7686, t7690, t7755, t7757, t7989, t8103);
        let (t8111, t8119, t8643) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1299(t3, t8110, t1458, t577, t7423, t7768, t7771, t7773, t2018, t3701);
    (t8097, t8103, t8107, t8110, t8111, t8119, t8643)
}
