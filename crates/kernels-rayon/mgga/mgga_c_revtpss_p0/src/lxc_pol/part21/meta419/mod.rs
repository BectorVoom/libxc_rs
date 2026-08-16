//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta419 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1904;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1905;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1906;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta419(t10416: f64, t1312: f64, t13425: f64, t13426: f64, t13429: f64, t13435: f64, t13440: f64, t13514: f64, t1518: f64, t2322: f64, t2371: f64, t4248: f64, t4292: f64, t5523: f64, t670: f64, t1843: f64, t1310: f64, t3813: f64, t5517: f64, t508: f64, t1453: f64, t1502: f64, t1519: f64, t2328: f64, t2372: f64, t4254: f64, t4257: f64, t4293: f64, t4297: f64, t5528: f64, t569: f64, t651: f64, t30: f64, t1468: f64, t9335: f64, t2: f64, t3833: f64, t580: f64, t605: f64, t22: f64, t2257: f64, t3834: f64, t513: f64, t5549: f64, t5552: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t13517 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1904(t10416, t1312, t13425, t13426, t13429, t13435, t13440, t13514, t1518, t2322, t2371, t4248, t4292, t5523, t670);
        let (t13521, t13532, t13537, t13540, t13544, t13547) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1905(t1843, t2371, t1310, t4292, t1518, t3813, t5517, t670, t13514, t508, t10416, t13435, t13517, t1453, t1502, t1519, t2322, t2328, t2372, t4248, t4254, t4257, t4293, t4297, t5528, t569, t651);
        let (t13550, t13553, t13554, t13564) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1906(t30, t1468, t9335, t2, t3833, t580, t605, t22, t2257, t3834, t513, t5549, t5552, zeta_threshold);
    (t13517, t13521, t13532, t13537, t13540, t13544, t13547, t13550, t13553, t13554, t13564)
}
