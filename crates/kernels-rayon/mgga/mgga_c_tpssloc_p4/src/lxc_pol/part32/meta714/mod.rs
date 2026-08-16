//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta714 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2243;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2244;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2245;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2246;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2247;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2248;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2249;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2250;
use chunk8::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2251;
use chunk9::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2252;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta714(t25083: f64, t4166: f64, t4184: f64, t16932: f64, t25084: f64, t16937: f64, t16907: f64, t23146: f64, t17009: f64, t17013: f64, t25111: f64, t7496: f64, t87447: f64, t87198: f64, t98610: f64, t98612: f64, t98614: f64, t98616: f64, t98618: f64, t98620: f64, t98622: f64, t98624: f64, t98626: f64, t22690: f64, t23122: f64, t5544: f64, t841: f64, t23097: f64, t5617: f64, t776: f64, t815: f64, t1510: f64, t4233: f64, t6605: f64, t232: f64, t58688: f64, t5612: f64, t1509: f64, t4119: f64, t67783: f64, t16888: f64, t16969: f64, t25146: f64, t4236: f64, t23053: f64, t5614: f64, t16859: f64, t6614: f64, t16673: f64, t6613: f64, t831: f64, t81736: f64, t81743: f64, t87206: f64, t87212: f64, t87213: f64, t28359: f64, t838: f64, t23069: f64, t5572: f64, t23062: f64, t28383: f64, t20986: f64, t2628: f64, t828: f64, t17004: f64, t6581: f64, t16662: f64, t1894: f64, t236: f64, t6591: f64, t5568: f64, t81956: f64, t28389: f64, t81963: f64, t81764: f64, t81789: f64, t81808: f64, t87234: f64, t87248: f64, t87256: f64, t87263: f64, t87271: f64, t87273: f64, t92597: f64, t25068: f64, t4257: f64, t16853: f64, t6621: f64, t16946: f64, t16951: f64, t5619: f64, t23083: f64, t28356: f64, t25093: f64, t87504: f64, t25115: f64, t87451: f64, t23133: f64, t5628: f64, t23041: f64, t1512: f64, t87261: f64, t81850: f64, t81853: f64, t87292: f64, t87293: f64, t87301: f64, t87306: f64, t92633: f64, t16944: f64, t25119: f64, t28372: f64, t28395: f64, t81782: f64, t81783: f64, t5587: f64, t81803: f64, t87295: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98629, t98631, t98633, t98635, t98637, t98639, t98642) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2243(t25083, t4166, t4184, t16932, t25084, t16937, t16907, t23146, t17009, t17013, t25111, t7496, t87447);
        let t98644 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2244(t87198, t98610, t98612, t98614, t98616, t98618, t98620, t98622, t98624, t98626, t98629, t98631, t98633, t98635, t98637, t98639, t98642);
        let (t98647, t98651, t98655, t98659) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2245(t22690, t23122, t5544, t841, t23097, t5617, t776, t815, t1510, t4233, t6605, t232, t58688);
        let (t98663, t98668, t98672, t98674, t98676, t98678) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2246(t23097, t5612, t776, t815, t1509, t232, t4119, t67783, t16888, t23146, t16969, t25146, t4236);
        let t98688 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2247(t23053, t5614, t16859, t6614, t16673, t6613, t831, t81736, t81743, t87206, t87212, t87213, t98647, t98651, t98655, t98659, t98663, t98668, t98672, t98674, t98676, t98678);
        let (t98690, t98694, t98696, t98701, t98703) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2248(t28359, t838, t23069, t5572, t23062, t28383, t20986, t2628, t6605, t828, t17004, t6581);
        let t98713 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2249(t16662, t1894, t236, t6591, t5568, t81956, t28389, t81963, t81764, t81789, t81808, t87234, t87248, t87256, t87263, t87271, t87273, t92597, t98690, t98694, t98696, t98701, t98703);
        let (t98715, t98717, t98719, t98721, t98723, t98725, t98728) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2250(t25068, t4257, t16853, t6621, t16946, t16951, t23053, t5619, t23083, t28356, t25093, t7496, t87504);
        let t98740 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2251(t25115, t7496, t87451, t23133, t5628, t23041, t5614, t1512, t87261, t81850, t81853, t87292, t87293, t87301, t87306, t92633, t98715, t98717, t98719, t98721, t98723, t98725, t98728);
        let (t98744, t98746, t98750, t98752, t98754) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2252(t16944, t25119, t841, t23083, t28372, t28395, t81782, t81783, t5587, t81803, t1512, t87295);
    (t98644, t98688, t98713, t98740, t98744, t98746, t98750, t98752, t98754)
}
