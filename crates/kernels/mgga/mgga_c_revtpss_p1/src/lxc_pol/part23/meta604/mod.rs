//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta604 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2257;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2258;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2259;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta604<F: Float>(t24129: F, t24176: F, t1079: F, t1076: F, t11201: F, t16284: F, t1652: F, t1680: F, t1696: F, t20175: F, t20191: F, t23959: F, t24044: F, t24048: F, t24061: F, t24068: F, t3058: F, t342: F, t386: F, t4747: F, t4752: F, t4935: F, t6235: F, t6245: F, t6251: F, t6259: F, t6351: F, t6393: F, t23628: F, t1102: F, t11108: F, t198: F, t23562: F, t23564: F, t23567: F, t23570: F, t23571: F, t23651: F, t23665: F, t23698: F, t23769: F, t23772: F, t23816: F, t23818: F, t336: F, t30: F, t265: F, t393: F, t23436: F, t23560: F, t1468: F, t1469: F, t1587: F, t1704: F, t22670: F, t22671: F, t395: F, t45: F, t5824: F, t5825: F, t6084: F, t6405: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F) {
        let (t24177, t24178, t24185) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2257::<F>(t24129, t24176, t1079, t1076, t11201, t16284, t1652, t1680, t1696, t20175, t20191, t23959, t24044, t24048, t24061, t24068, t3058, t342, t386, t4747, t4752, t4935, t6235, t6245, t6251, t6259, t6351, t6393);
        let (t24186, t24190) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2258::<F>(t23628, t24185, t1102, t11108, t198, t23562, t23564, t23567, t23570, t23571, t23651, t23665, t23698, t23769, t23772, t23816, t23818, t336);
        let (t24192, t24202) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2259::<F>(t30, t265, t393, t23436, t23560, t24190, t1468, t1469, t1587, t1704, t22670, t22671, t395, t45, t5824, t5825, t6084, t6405, dens_threshold, rho0, zeta_threshold);
    (t24177, t24178, t24186, t24192, t24202)
}
