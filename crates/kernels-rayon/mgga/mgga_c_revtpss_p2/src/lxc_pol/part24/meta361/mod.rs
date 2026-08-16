//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta361 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1231;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1232;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1233;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta361(t24129: f64, t24176: f64, t1079: f64, t1076: f64, t11201: f64, t16284: f64, t1652: f64, t1680: f64, t1696: f64, t20175: f64, t20191: f64, t23959: f64, t24044: f64, t24048: f64, t24061: f64, t24068: f64, t3058: f64, t342: f64, t386: f64, t4747: f64, t4752: f64, t4935: f64, t6235: f64, t6245: f64, t6251: f64, t6259: f64, t6351: f64, t6393: f64, t23628: f64, t1102: f64, t11108: f64, t198: f64, t23562: f64, t23564: f64, t23567: f64, t23570: f64, t23571: f64, t23651: f64, t23665: f64, t23698: f64, t23769: f64, t23772: f64, t23816: f64, t23818: f64, t336: f64, t30: f64, t265: f64, t393: f64, t23436: f64, t23560: f64, t1468: f64, t1469: f64, t1587: f64, t1704: f64, t22670: f64, t22671: f64, t395: f64, t45: f64, t5824: f64, t5825: f64, t6084: f64, t6405: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
        let (t24177, t24178, t24185) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1231(t24129, t24176, t1079, t1076, t11201, t16284, t1652, t1680, t1696, t20175, t20191, t23959, t24044, t24048, t24061, t24068, t3058, t342, t386, t4747, t4752, t4935, t6235, t6245, t6251, t6259, t6351, t6393);
        let (t24186, t24190) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1232(t23628, t24185, t1102, t11108, t198, t23562, t23564, t23567, t23570, t23571, t23651, t23665, t23698, t23769, t23772, t23816, t23818, t336);
        let (t24192, t24202) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1233(t30, t265, t393, t23436, t23560, t24190, t1468, t1469, t1587, t1704, t22670, t22671, t395, t45, t5824, t5825, t6084, t6405, dens_threshold, rho0, zeta_threshold);
    (t24177, t24178, t24186, t24192, t24202)
}
