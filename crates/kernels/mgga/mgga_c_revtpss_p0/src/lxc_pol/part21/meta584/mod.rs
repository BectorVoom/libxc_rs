//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta584 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2298;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2299;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2300;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta584<F: Float>(t3801: F, t5501: F, t12587: F, t1832: F, t1298: F, t16786: F, t16788: F, t16790: F, t16809: F, t16814: F, t16834: F, t16837: F, t16839: F, t16842: F, t16844: F, t16846: F, t16945: F, t17094: F, t17160: F, t17162: F, t17166: F, t17168: F, t3794: F, t3798: F, t5023: F, t5505: F, t33: F, t265: F, t502: F, t15083: F, t18127: F, t1113: F, t1304: F, t13312: F, t1469: F, t15093: F, t15094: F, t15096: F, t1587: F, t1711: F, t1837: F, t2258: F, t2838: F, t3351: F, t3805: F, t4186: F, t4560: F, t504: F, t5509: F, t57: F, t606: F, dens_threshold: F, rho1: F, zeta_threshold: F, t16630: F, t2371: F, t94: F, t118: F, t1310: F, t1315: F, t13425: F, t13426: F, t13429: F, t14310: F, t1519: F, t1843: F, t1847: F, t1911: F, t2320: F, t2322: F, t2331: F, t3821: F, t4151: F, t4246: F, t4248: F, t4254: F, t4257: F, t4293: F, t508: F, t511: F, t5517: F, t5787: F, t649: F, t671: F) -> (F, F, F, F, F, F) {
        let (t18128, t18134, t18138) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2298::<F>(t3801, t5501, t12587, t1832, t1298, t16786, t16788, t16790, t16809, t16814, t16834, t16837, t16839, t16842, t16844, t16846, t16945, t17094, t17160, t17162, t17166, t17168, t3794, t3798, t5023, t5505);
        let (t18140, t18152) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2299::<F>(t33, t265, t502, t15083, t18127, t18138, t1113, t1304, t13312, t1469, t15093, t15094, t15096, t1587, t1711, t1837, t2258, t2838, t3351, t3805, t4186, t4560, t504, t5509, t57, t606, dens_threshold, rho1, zeta_threshold);
        let (t18153, t18163, t18176) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2300::<F>(t16630, t18152, t2371, t94, t118, t1310, t1315, t13425, t13426, t13429, t14310, t1519, t1843, t1847, t1911, t2320, t2322, t2331, t3821, t4151, t4246, t4248, t4254, t4257, t4293, t508, t511, t5517, t5787, t649, t671);
    (t18128, t18134, t18140, t18153, t18163, t18176)
}
