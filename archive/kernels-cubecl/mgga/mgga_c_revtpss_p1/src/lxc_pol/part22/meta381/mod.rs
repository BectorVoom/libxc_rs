//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta381 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1943;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1944;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1945;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1946;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta381<F: Float>(t1843: F, t2371: F, t1310: F, t4292: F, t1518: F, t3813: F, t5517: F, t670: F, t13514: F, t508: F, t10416: F, t13435: F, t13517: F, t1453: F, t1502: F, t1519: F, t2322: F, t2328: F, t2372: F, t4248: F, t4254: F, t4257: F, t4293: F, t4297: F, t5528: F, t569: F, t651: F, t30: F, t1468: F, t9335: F, t2: F, t3833: F, t580: F, t605: F, t22: F, t2257: F, t3834: F, t513: F, t5549: F, t5552: F, zeta_threshold: F, t33: F, t1711: F, t9350: F, t3841: F, t1113: F, t3351: F, t3842: F, t516: F, t5557: F, t5560: F, t162: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13521, t13532, t13537, t13540, t13544, t13547) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1943::<F>(t1843, t2371, t1310, t4292, t1518, t3813, t5517, t670, t13514, t508, t10416, t13435, t13517, t1453, t1502, t1519, t2322, t2328, t2372, t4248, t4254, t4257, t4293, t4297, t5528, t569, t651);
        let (t13550, t13554, t13564) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1944::<F>(t30, t1468, t9335, t2, t3833, t580, t605, t22, t2257, t3834, t513, t5549, t5552, zeta_threshold);
        let (t13565, t13569, t13579) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1945::<F>(t33, t1711, t9350, t2, t3841, t1113, t580, t22, t3351, t3842, t516, t5557, t5560, zeta_threshold);
        let t13581 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1946::<F>(t13564, t13579, t162);
    (t13521, t13532, t13537, t13540, t13544, t13547, t13550, t13554, t13565, t13569, t13581)
}
