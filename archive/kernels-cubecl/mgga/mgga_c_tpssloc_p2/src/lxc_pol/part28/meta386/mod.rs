//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta386 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1504;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1505;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1506;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1507;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1508;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1509;
use chunk6::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1510;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta386<F: Float>(t15030: F, t15785: F, t1241: F, t1251: F, t5088: F, t3598: F, t1760: F, t3599: F, t11606: F, t225: F, t4941: F, t1751: F, t3481: F, t3630: F, t1238: F, t1252: F, t14972: F, t14980: F, t3487: F, t3593: F, t3600: F, t3631: F, t498: F, t5055: F, t5060: F, t5089: F, t1720: F, t3590: F, t15425: F, t491: F, t1235: F, t4940: F, t5053: F, t1190: F, t5052: F, t15771: F, t466: F, t11613: F, t11925: F, t11928: F, t1761: F, t4945: F, t11947: F, t1763: F, t1256: F, t14963: F, t14969: F, t14971: F, t15038: F, t15040: F, t15043: F, t15046: F, t15048: F, t15050: F, t15053: F, t15056: F, t15059: F, t15063: F, t15066: F, t15070: F, t15235: F, t15237: F, t193: F, t336: F, t3633: F, t3637: F, t4700: F, t5095: F, t28: F, t265: F, t504: F, t13493: F, t14959: F, t1081: F, t1260: F, t12606: F, t13503: F, t13504: F, t13506: F, t1409: F, t1534: F, t1649: F, t1768: F, t2250: F, t2756: F, t3231: F, t3644: F, t3966: F, t4324: F, t506: F, t5099: F, t52: F, t607: F, dens_threshold: F, rho1: F, zeta_threshold: F, t14687: F, t3701: F, t5356: F, t3719: F, t5127: F, t5168: F, t588: F, t592: F, t5166: F, t5187: F, t571: F) -> (F, F, F, F, F, F, F) {
        let (t15787, t15790, t15794, t15797, t15800) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1504::<F>(t15030, t15785, t1241, t1251, t5088, t3598, t1760, t3599, t11606, t225, t4941, t1751, t3481);
        let t15806 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1505::<F>(t1760, t3630, t3598, t1238, t1252, t14972, t14980, t15787, t15790, t15794, t15797, t15800, t3487, t3593, t3600, t3631, t498, t5055, t5060, t5089);
        let (t15808, t15814, t15816, t15820, t15823, t15831) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1506::<F>(t1720, t3590, t15425, t491, t1235, t4940, t225, t5053, t1190, t5052, t15771, t466);
        let t15833 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1507::<F>(t11613, t11925, t11928, t1252, t15808, t15814, t15816, t15820, t15823, t15831, t1761, t3487, t3593, t3600, t3631, t4945, t498, t5060, t5089);
        let t15842 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1508::<F>(t15806, t15833, t11947, t1763, t1256, t14963, t14969, t14971, t15038, t15040, t15043, t15046, t15048, t15050, t15053, t15056, t15059, t15063, t15066, t15070, t15235, t15237, t193, t336, t3633, t3637, t4700, t5095);
        let t15856 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1509::<F>(t28, t265, t504, t13493, t14959, t15842, t1081, t1260, t12606, t13503, t13504, t13506, t1409, t1534, t1649, t1768, t2250, t2756, t3231, t3644, t3966, t4324, t506, t5099, t52, t607, dens_threshold, rho1, zeta_threshold);
        let (t15857, t15868, t15872, t15876, t15878, t15880, t15883) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1510::<F>(t14687, t15856, t3701, t5356, t3719, t5127, t5168, t588, t592, t5166, t5187, t571);
    (t15857, t15868, t15872, t15876, t15878, t15880, t15883)
}
