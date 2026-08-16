//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta745 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2473;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2474;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2475;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2476;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2477;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2478;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2479;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2480;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta745(t1020: f64, t21595: f64, t248: f64, t3101: f64, t14511: f64, t17655: f64, t10883: f64, t21403: f64, t1041: f64, t21130: f64, t42592: f64, t21594: f64, t376: f64, t1023: f64, t10413: f64, t14077: f64, t21516: f64, t21532: f64, t3039: f64, t3048: f64, t3070: f64, t3071: f64, t42483: f64, t42546: f64, t4347: f64, t4582: f64, t48611: f64, t48670: f64, t48674: f64, t5681: f64, t5867: f64, t5869: f64, t61866: f64, t70086: f64, t70122: f64, t10422: f64, t21519: f64, t10403: f64, t10408: f64, t10904: f64, t21487: f64, t49662: f64, t5677: f64, t61916: f64, t61919: f64, t61923: f64, t61929: f64, t61940: f64, t61975: f64, t61977: f64, t70082: f64, t1616: f64, t17187: f64, t17980: f64, t42552: f64, t4575: f64, t4650: f64, t49691: f64, t49693: f64, t50193: f64, t61950: f64, t61981: f64, t62013: f64, t62032: f64, t62038: f64, t3966: f64, t20217: f64, t607: f64, t10949: f64, t14211: f64, t21538: f64, t21562: f64, t2960: f64, t3130: f64, t4588: f64, t4596: f64, t4600: f64, t61736: f64, t61739: f64, t62091: f64, t62137: f64, t62148: f64, t62150: f64, t62152: f64, t135: f64, t21561: f64, t973: f64, t10390: f64, t14207: f64, t17712: f64, t17732: f64, t17984: f64, t21526: f64, t21566: f64, t369: f64, t378: f64, t42505: f64, t50265: f64, t5878: f64, t62164: f64, t62177: f64, t62183: f64, t68: f64, t70012: f64, t21525: f64, t10876: f64, t14508: f64, t1539: f64, t17670: f64, t17714: f64, t17890: f64, t17960: f64, t21118: f64, t21398: f64, t21512: f64, t42565: f64, t4644: f64, t47779: f64, t62210: f64, t62234: f64, t70330: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t70346, t70351, t70363, t70389, t70391) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2473(t1020, t21595, t248, t3101, t14511, t17655, t10883, t21403, t1041, t21130, t42592, t21594, t376);
        let t70396 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2474(t1023, t10413, t14077, t21516, t21532, t3039, t3048, t3070, t3071, t42483, t42546, t4347, t4582, t48611, t48670, t48674, t5681, t5867, t5869, t61866, t70086, t70122, t70389, t70391);
        let t70414 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2475(t10422, t21519, t3070, t10403, t10408, t10904, t21487, t49662, t5677, t61916, t61919, t61923, t61929, t61940, t61975, t61977, t70082);
        let t70432 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2476(t10408, t1616, t17187, t17980, t3070, t3071, t42552, t4575, t4650, t49691, t49693, t50193, t5677, t61950, t61981, t62013, t62032, t62038);
        let (t70442, t70458) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2477(t1616, t3966, t20217, t607);
        let t70481 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2478(t1041, t10949, t14211, t21487, t21538, t21562, t2960, t3130, t4582, t4588, t4596, t4600, t61736, t61739, t62091, t62137, t62148, t62150, t62152, t70458);
        let t70509 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2479(t135, t21561, t973, t10390, t10413, t14207, t17712, t17732, t17984, t21526, t21566, t3071, t3130, t369, t378, t42505, t4347, t4582, t50265, t5869, t5878, t62164, t62177, t62183, t68, t70012);
        let t70539 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2480(t10403, t10422, t21525, t1023, t10408, t1041, t10876, t14508, t1539, t17670, t17714, t17732, t17890, t17960, t21118, t21398, t21512, t3048, t3070, t3071, t42565, t4582, t4644, t47779, t62210, t62234, t70330);
    (t70346, t70351, t70363, t70391, t70396, t70414, t70432, t70442, t70458, t70481, t70509, t70539)
}
