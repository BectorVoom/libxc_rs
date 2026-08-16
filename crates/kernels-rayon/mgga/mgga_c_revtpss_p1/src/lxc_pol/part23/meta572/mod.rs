//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta572 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2167;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2168;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2169;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta572(t1868: f64, t22486: f64, t5532: f64, t6836: f64, t1907: f64, t198: f64, t22483: f64, t22813: f64, t22925: f64, t22926: f64, t5536: f64, t5541: f64, t566: f64, t9514: f64, t9517: f64, t9521: f64, t9524: f64, t9546: f64, t9569: f64, t9574: f64, t9577: f64, t9588: f64, t6781: f64, t21937: f64, t22466: f64, t22928: f64, t22929: f64, t22930: f64, t22931: f64, t22932: f64, t4139: f64, t532: f64, t6816: f64, t9542: f64, t9593: f64, t9598: f64, t9854: f64, t9857: f64, t9865: f64, t9868: f64, t22767: f64, t23063: f64, t14312: f64, t18301: f64, t1522: f64, t18263: f64, t14328: f64, t14334: f64, t10552: f64, t10554: f64, t2403: f64, t4546: f64, t5962: f64, t9278: f64, t9308: f64, t9316: f64, t9329: f64, t9333: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23068, t23071, t23077) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2167(t1868, t22486, t5532, t6836, t1907, t198, t22483, t22813, t22925, t22926, t5536, t5541, t566, t9514, t9517, t9521, t9524, t9546, t9569, t9574, t9577, t9588);
        let (t23087, t23092) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2168(t1907, t6781, t1868, t198, t21937, t22466, t22928, t22929, t22930, t22931, t22932, t4139, t532, t5532, t6816, t9542, t9593, t9598, t9854, t9857, t9865, t9868);
        let (t23094, t23096, t23097, t23102, t23103, t23104, t23105) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2169(t22767, t23063, t23077, t23092, t14312, t18301, t1522, t18263, t14328, t14334, t10552, t10554, t2403, t4546, t5962, t9278, t9308, t9316, t9329, t9333);
    (t23068, t23071, t23087, t23094, t23096, t23097, t23102, t23103, t23104, t23105)
}
