//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta515 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2020;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2021;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta515<F: Float>(t1222: F, t1266: F, t12784: F, t12855: F, t17437: F, t21121: F, t21126: F, t21129: F, t21134: F, t21137: F, t21140: F, t21143: F, t5304: F, t5309: F, t5313: F, t5373: F, t5391: F, t6640: F, t1264: F, t20272: F, t247: F, t5405: F, t6429: F, t3626: F, t6425: F, t1794: F, t5245: F, t1250: F, t3720: F, t140: F, t6652: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t21146 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2020::<F>(t1222, t1266, t12784, t12855, t17437, t21121, t21126, t21129, t21134, t21137, t21140, t21143, t5304, t5309, t5313, t5373, t5391, t6640);
        let (t21153, t21156, t21157, t21160, t21161, t21164, t21165, t21166, t21169) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2021::<F>(t1264, t20272, t247, t5405, t6429, t3626, t6425, t1794, t5245, t1250, t3720, t140, t6652);
    (t21146, t21153, t21156, t21157, t21160, t21161, t21164, t21165, t21166, t21169)
}
