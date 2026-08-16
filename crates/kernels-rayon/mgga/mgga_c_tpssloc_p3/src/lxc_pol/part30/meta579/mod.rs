//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta579 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1955;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1956;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1957;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1958;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta579(t1976: f64, t5493: f64, t1458: f64, t7670: f64, t19596: f64, t2019: f64, t1983: f64, t7458: f64, t7468: f64, t1873: f64, t6287: f64, t652: f64, t1442: f64, t1774: f64, t1849: f64, t1869: f64, t28819: f64, t28822: f64, t28825: f64, t28829: f64, t28833: f64, t28837: f64, t28841: f64, t28843: f64, t4028: f64, t5450: f64, t5457: f64, t7451: f64, t7472: f64, t7681: f64, t28816: f64, t3: f64, t20162: f64, t16524: f64, t7769: f64, t5371: f64, t7467: f64, t5456: f64, t576: f64, t3941: f64, t1401: f64, t28017: f64, t23880: f64, t26523: f64, t577: f64, t7010: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28852, t28855, t28860, t28861, t28863, t28864, t28866) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1955(t1976, t5493, t1458, t7670, t19596, t2019, t1983, t7458, t7468, t1873, t6287, t652);
        let t28867 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1956(t1442, t1774, t1849, t1869, t1976, t28819, t28822, t28825, t28829, t28833, t28837, t28841, t28843, t28852, t28855, t28861, t28863, t28866, t4028, t5450, t5457, t6287, t652, t7451, t7472, t7670, t7681);
        let (t28868, t28869, t28888, t28890, t28892, t28893, t28895, t28896) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1957(t28816, t28867, t3, t1873, t20162, t16524, t7769, t5371, t7467, t5456, t576, t1458);
        let (t28899, t28904) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1958(t28896, t3941, t1873, t5493, t1401, t28017, t1458, t23880, t26523, t28868, t28888, t28890, t28892, t28895, t5456, t577, t7010);
    (t28852, t28855, t28860, t28864, t28868, t28869, t28893, t28896, t28899, t28904)
}
