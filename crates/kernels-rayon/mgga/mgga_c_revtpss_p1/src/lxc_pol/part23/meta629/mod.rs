//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta629 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2321;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2322;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2323;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta629(t33: f64, t265: f64, t502: f64, t23436: f64, t24476: f64, t25030: f64, t1469: f64, t1587: f64, t1711: f64, t1837: f64, t22671: f64, t22783: f64, t504: f64, t57: f64, t5825: f64, t6084: f64, t6416: f64, t6757: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t24202: f64, t1518: f64, t6765: f64, t118: f64, t1502: f64, t1519: f64, t18245: f64, t1843: f64, t1847: f64, t1911: f64, t22578: f64, t22634: f64, t22639: f64, t22747: f64, t22758: f64, t23094: f64, t4248: f64, t508: f64, t511: f64, t569: f64, t5877: f64, t5884: f64, t5887: f64, t5921: f64, t651: f64, t6773: f64, t6934: f64, t7732: f64, t3: f64, t5883: f64, t5801: f64, t5920: f64, t117: f64, t22633: f64, t1916: f64, t1918: f64, t572: f64, t573: f64, t6941: f64, t6945: f64, t6948: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25032, t25042) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2321(t33, t265, t502, t23436, t24476, t25030, t1469, t1587, t1711, t1837, t22671, t22783, t504, t57, t5825, t6084, t6416, t6757, dens_threshold, rho1, zeta_threshold);
        let (t25043, t25045, t25048) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2322(t24202, t25042, t1518, t6765, t118, t1502, t1519, t18245, t1843, t1847, t1911, t22578, t22634, t22639, t22747, t22758, t23094, t4248, t508, t511, t569, t5877, t5884, t5887, t5921, t651, t6773, t6934, t7732);
        let (t25049, t25055, t25063, t25066, t25069, t25072) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2323(t25048, t3, t1518, t5883, t5801, t5920, t117, t22633, t1916, t1918, t572, t573, t6941, t6945, t6948, param_d);
    (t25032, t25043, t25045, t25048, t25049, t25055, t25063, t25066, t25069, t25072)
}
