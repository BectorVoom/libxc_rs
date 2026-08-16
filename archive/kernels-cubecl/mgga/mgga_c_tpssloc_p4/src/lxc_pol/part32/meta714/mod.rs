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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta714<F: Float>(t25083: F, t4166: F, t4184: F, t16932: F, t25084: F, t16937: F, t16907: F, t23146: F, t17009: F, t17013: F, t25111: F, t7496: F, t87447: F, t87198: F, t98610: F, t98612: F, t98614: F, t98616: F, t98618: F, t98620: F, t98622: F, t98624: F, t98626: F, t22690: F, t23122: F, t5544: F, t841: F, t23097: F, t5617: F, t776: F, t815: F, t1510: F, t4233: F, t6605: F, t232: F, t58688: F, t5612: F, t1509: F, t4119: F, t67783: F, t16888: F, t16969: F, t25146: F, t4236: F, t23053: F, t5614: F, t16859: F, t6614: F, t16673: F, t6613: F, t831: F, t81736: F, t81743: F, t87206: F, t87212: F, t87213: F, t28359: F, t838: F, t23069: F, t5572: F, t23062: F, t28383: F, t20986: F, t2628: F, t828: F, t17004: F, t6581: F, t16662: F, t1894: F, t236: F, t6591: F, t5568: F, t81956: F, t28389: F, t81963: F, t81764: F, t81789: F, t81808: F, t87234: F, t87248: F, t87256: F, t87263: F, t87271: F, t87273: F, t92597: F, t25068: F, t4257: F, t16853: F, t6621: F, t16946: F, t16951: F, t5619: F, t23083: F, t28356: F, t25093: F, t87504: F, t25115: F, t87451: F, t23133: F, t5628: F, t23041: F, t1512: F, t87261: F, t81850: F, t81853: F, t87292: F, t87293: F, t87301: F, t87306: F, t92633: F, t16944: F, t25119: F, t28372: F, t28395: F, t81782: F, t81783: F, t5587: F, t81803: F, t87295: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t98629, t98631, t98633, t98635, t98637, t98639, t98642) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2243::<F>(t25083, t4166, t4184, t16932, t25084, t16937, t16907, t23146, t17009, t17013, t25111, t7496, t87447);
        let t98644 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2244::<F>(t87198, t98610, t98612, t98614, t98616, t98618, t98620, t98622, t98624, t98626, t98629, t98631, t98633, t98635, t98637, t98639, t98642);
        let (t98647, t98651, t98655, t98659) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2245::<F>(t22690, t23122, t5544, t841, t23097, t5617, t776, t815, t1510, t4233, t6605, t232, t58688);
        let (t98663, t98668, t98672, t98674, t98676, t98678) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2246::<F>(t23097, t5612, t776, t815, t1509, t232, t4119, t67783, t16888, t23146, t16969, t25146, t4236);
        let t98688 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2247::<F>(t23053, t5614, t16859, t6614, t16673, t6613, t831, t81736, t81743, t87206, t87212, t87213, t98647, t98651, t98655, t98659, t98663, t98668, t98672, t98674, t98676, t98678);
        let (t98690, t98694, t98696, t98701, t98703) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2248::<F>(t28359, t838, t23069, t5572, t23062, t28383, t20986, t2628, t6605, t828, t17004, t6581);
        let t98713 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2249::<F>(t16662, t1894, t236, t6591, t5568, t81956, t28389, t81963, t81764, t81789, t81808, t87234, t87248, t87256, t87263, t87271, t87273, t92597, t98690, t98694, t98696, t98701, t98703);
        let (t98715, t98717, t98719, t98721, t98723, t98725, t98728) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2250::<F>(t25068, t4257, t16853, t6621, t16946, t16951, t23053, t5619, t23083, t28356, t25093, t7496, t87504);
        let t98740 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2251::<F>(t25115, t7496, t87451, t23133, t5628, t23041, t5614, t1512, t87261, t81850, t81853, t87292, t87293, t87301, t87306, t92633, t98715, t98717, t98719, t98721, t98723, t98725, t98728);
        let (t98744, t98746, t98750, t98752, t98754) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2252::<F>(t16944, t25119, t841, t23083, t28372, t28395, t81782, t81783, t5587, t81803, t1512, t87295);
    (t98644, t98688, t98713, t98740, t98744, t98746, t98750, t98752, t98754)
}
