//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta584 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2298;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2299;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2300;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta584(t3801: f64, t5501: f64, t12587: f64, t1832: f64, t1298: f64, t16786: f64, t16788: f64, t16790: f64, t16809: f64, t16814: f64, t16834: f64, t16837: f64, t16839: f64, t16842: f64, t16844: f64, t16846: f64, t16945: f64, t17094: f64, t17160: f64, t17162: f64, t17166: f64, t17168: f64, t3794: f64, t3798: f64, t5023: f64, t5505: f64, t33: f64, t265: f64, t502: f64, t15083: f64, t18127: f64, t1113: f64, t1304: f64, t13312: f64, t1469: f64, t15093: f64, t15094: f64, t15096: f64, t1587: f64, t1711: f64, t1837: f64, t2258: f64, t2838: f64, t3351: f64, t3805: f64, t4186: f64, t4560: f64, t504: f64, t5509: f64, t57: f64, t606: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t16630: f64, t2371: f64, t94: f64, t118: f64, t1310: f64, t1315: f64, t13425: f64, t13426: f64, t13429: f64, t14310: f64, t1519: f64, t1843: f64, t1847: f64, t1911: f64, t2320: f64, t2322: f64, t2331: f64, t3821: f64, t4151: f64, t4246: f64, t4248: f64, t4254: f64, t4257: f64, t4293: f64, t508: f64, t511: f64, t5517: f64, t5787: f64, t649: f64, t671: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t18128, t18134, t18138) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2298(t3801, t5501, t12587, t1832, t1298, t16786, t16788, t16790, t16809, t16814, t16834, t16837, t16839, t16842, t16844, t16846, t16945, t17094, t17160, t17162, t17166, t17168, t3794, t3798, t5023, t5505);
        let (t18140, t18152) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2299(t33, t265, t502, t15083, t18127, t18138, t1113, t1304, t13312, t1469, t15093, t15094, t15096, t1587, t1711, t1837, t2258, t2838, t3351, t3805, t4186, t4560, t504, t5509, t57, t606, dens_threshold, rho1, zeta_threshold);
        let (t18153, t18163, t18176) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2300(t16630, t18152, t2371, t94, t118, t1310, t1315, t13425, t13426, t13429, t14310, t1519, t1843, t1847, t1911, t2320, t2322, t2331, t3821, t4151, t4246, t4248, t4254, t4257, t4293, t508, t511, t5517, t5787, t649, t671);
    (t18128, t18134, t18140, t18153, t18163, t18176)
}
