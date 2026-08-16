//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta407 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1657;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1658;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1659;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1660;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta407(t1720: f64, t3590: f64, t15425: f64, t491: f64, t1235: f64, t4940: f64, t225: f64, t5053: f64, t1190: f64, t5052: f64, t15771: f64, t466: f64, t11613: f64, t11925: f64, t11928: f64, t1252: f64, t1761: f64, t3487: f64, t3593: f64, t3600: f64, t3631: f64, t4945: f64, t498: f64, t5060: f64, t5089: f64, t15806: f64, t11947: f64, t1763: f64, t1256: f64, t14963: f64, t14969: f64, t14971: f64, t15038: f64, t15040: f64, t15043: f64, t15046: f64, t15048: f64, t15050: f64, t15053: f64, t15056: f64, t15059: f64, t15063: f64, t15066: f64, t15070: f64, t15235: f64, t15237: f64, t193: f64, t336: f64, t3633: f64, t3637: f64, t4700: f64, t5095: f64, t28: f64, t265: f64, t504: f64, t13493: f64, t14959: f64, t1081: f64, t1260: f64, t12606: f64, t13503: f64, t13504: f64, t13506: f64, t1409: f64, t1534: f64, t1649: f64, t1768: f64, t2250: f64, t2756: f64, t3231: f64, t3644: f64, t3966: f64, t4324: f64, t506: f64, t5099: f64, t52: f64, t607: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15808, t15814, t15816, t15820, t15823, t15831) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1657(t1720, t3590, t15425, t491, t1235, t4940, t225, t5053, t1190, t5052, t15771, t466);
        let t15833 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1658(t11613, t11925, t11928, t1252, t15808, t15814, t15816, t15820, t15823, t15831, t1761, t3487, t3593, t3600, t3631, t4945, t498, t5060, t5089);
        let (t15834, t15842) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1659(t15806, t15833, t11947, t1763, t1256, t14963, t14969, t14971, t15038, t15040, t15043, t15046, t15048, t15050, t15053, t15056, t15059, t15063, t15066, t15070, t15235, t15237, t193, t336, t3633, t3637, t4700, t5095);
        let t15856 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1660(t28, t265, t504, t13493, t14959, t15842, t1081, t1260, t12606, t13503, t13504, t13506, t1409, t1534, t1649, t1768, t2250, t2756, t3231, t3644, t3966, t4324, t506, t5099, t52, t607, dens_threshold, rho1, zeta_threshold);
    (t15808, t15814, t15816, t15820, t15823, t15831, t15834, t15856)
}
