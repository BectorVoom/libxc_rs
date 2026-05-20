//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta329 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1146;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1147;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1148;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1149;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta329<F: Float>(t1868: F, t22486: F, t5532: F, t6836: F, t1907: F, t198: F, t22483: F, t22813: F, t22925: F, t22926: F, t5536: F, t5541: F, t566: F, t9514: F, t9517: F, t9521: F, t9524: F, t9546: F, t9569: F, t9574: F, t9577: F, t9588: F, t6781: F, t21937: F, t22466: F, t22928: F, t22929: F, t22930: F, t22931: F, t22932: F, t4139: F, t532: F, t6816: F, t9542: F, t9593: F, t9598: F, t9854: F, t9857: F, t9865: F, t9868: F, t22767: F, t23063: F, t14312: F, t18301: F, t1522: F, t18263: F, t14328: F, t14334: F, t10552: F, t10554: F, t2403: F, t4546: F, t5962: F, t9278: F, t9308: F, t9316: F, t9329: F, t9333: F, t14336: F, t14339: F, t1544: F, t18860: F, t5966: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t23077 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1146::<F>(t1868, t22486, t5532, t6836, t1907, t198, t22483, t22813, t22925, t22926, t5536, t5541, t566, t9514, t9517, t9521, t9524, t9546, t9569, t9574, t9577, t9588);
        let (t23087, t23092) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1147::<F>(t1907, t6781, t1868, t198, t21937, t22466, t22928, t22929, t22930, t22931, t22932, t4139, t532, t5532, t6816, t9542, t9593, t9598, t9854, t9857, t9865, t9868);
        let (t23094, t23096, t23097, t23102, t23103, t23104, t23105) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1148::<F>(t22767, t23063, t23077, t23092, t14312, t18301, t1522, t18263, t14328, t14334, t10552, t10554, t2403, t4546, t5962, t9278, t9308, t9316, t9329, t9333);
        let (t23106, t23110, t23111, t23114) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1149::<F>(t14336, t14339, t1544, t18860, t5966);
    (t23087, t23094, t23096, t23097, t23102, t23103, t23104, t23105, t23106, t23110, t23111, t23114)
}
