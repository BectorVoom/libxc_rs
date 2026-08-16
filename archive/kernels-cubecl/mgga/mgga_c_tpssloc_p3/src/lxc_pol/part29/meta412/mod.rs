//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta412 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1670;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1671;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1672;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1673;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta412<F: Float>(t25: F, t1788: F, t2225: F, t2221: F, t2223: F, t12130: F, t11987: F, t1408: F, t2: F, t3704: F, t1298: F, t15941: F, t16: F, t2249: F, t3665: F, t5170: F, t5173: F, t584: F, zeta_threshold: F, t28: F, t12000: F, t1649: F, t3711: F, t1302: F, t15956: F, t3231: F, t3673: F, t5178: F, t5181: F, t225: F, t5213: F, t1807: F, t3879: F, t5211: F, t1332: F, t5343: F, t1372: F, t1824: F, t5250: F, t5286: F, t562: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t15983, t15985, t15987, t15988, t16002) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1670::<F>(t25, t1788, t2225, t2221, t2223, t12130, t11987, t1408, t2, t3704, t1298, t15941, t16, t2249, t3665, t5170, t5173, t584, zeta_threshold);
        let t16016 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1671::<F>(t28, t12000, t1649, t2, t3711, t1302, t15956, t16, t3231, t3673, t5178, t5181, t584, zeta_threshold);
        let t16018 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1672::<F>(t16002, t16016);
        let (t16022, t16028, t16030, t16033, t16036, t16037, t16040) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1673::<F>(t225, t5213, t1807, t3879, t5211, t1332, t5343, t1372, t1824, t5250, t5286, t562);
    (t15983, t15985, t15987, t15988, t16018, t16022, t16028, t16030, t16033, t16036, t16037, t16040)
}
