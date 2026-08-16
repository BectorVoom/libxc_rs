//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta388 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1293;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1294;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1295;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta388<F: Float>(t33: F, t265: F, t502: F, t23436: F, t24476: F, t25030: F, t1469: F, t1587: F, t1711: F, t1837: F, t22671: F, t22783: F, t504: F, t57: F, t5825: F, t6084: F, t6416: F, t6757: F, dens_threshold: F, rho1: F, zeta_threshold: F, t24202: F, t1518: F, t6765: F, t118: F, t1502: F, t1519: F, t18245: F, t1843: F, t1847: F, t1911: F, t22578: F, t22634: F, t22639: F, t22747: F, t22758: F, t23094: F, t4248: F, t508: F, t511: F, t569: F, t5877: F, t5884: F, t5887: F, t5921: F, t651: F, t6773: F, t6934: F, t7732: F, t3: F, t5883: F, t5801: F, t5920: F, t117: F, t22633: F, t1916: F, t1918: F, t572: F, t573: F, t6941: F, t6945: F, t6948: F, param_d: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t25032, t25042) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1293::<F>(t33, t265, t502, t23436, t24476, t25030, t1469, t1587, t1711, t1837, t22671, t22783, t504, t57, t5825, t6084, t6416, t6757, dens_threshold, rho1, zeta_threshold);
        let (t25043, t25045, t25048) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1294::<F>(t24202, t25042, t1518, t6765, t118, t1502, t1519, t18245, t1843, t1847, t1911, t22578, t22634, t22639, t22747, t22758, t23094, t4248, t508, t511, t569, t5877, t5884, t5887, t5921, t651, t6773, t6934, t7732);
        let (t25049, t25055, t25063, t25066, t25069, t25072) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1295::<F>(t25048, t3, t1518, t5883, t5801, t5920, t117, t22633, t1916, t1918, t572, t573, t6941, t6945, t6948, param_d);
    (t25032, t25043, t25045, t25048, t25049, t25055, t25063, t25066, t25069, t25072)
}
