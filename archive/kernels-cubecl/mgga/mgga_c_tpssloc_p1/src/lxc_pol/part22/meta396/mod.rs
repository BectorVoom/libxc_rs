//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta396 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1688;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1689;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta396<F: Float>(t11721: F, t1215: F, t18300: F, t4582: F, t4978: F, t1222: F, t6170: F, t6158: F, t6165: F, t11644: F, t11649: F, t11719: F, t11728: F, t15446: F, t15448: F, t15450: F, t15452: F, t15503: F, t15507: F, t18297: F, t488: F, t4974: F, t4980: F, t4984: F, t5005: F, t5416: F, t972: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t18301, t18302, t18303, t18306, t18307, t18310, t18312, t18314, t18316) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1688::<F>(t11721, t1215, t18300, t4582, t4978, t1222, t6170, t6158, t6165, t11644, t11649, t11719, t11728, t15446, t15448, t15450, t15452, t15503, t15507, t18297, t488, t4974, t4980, t4984, t5005);
        let t18321 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1689::<F>(t5416, t972);
    (t18301, t18302, t18303, t18306, t18307, t18310, t18312, t18314, t18316, t18321)
}
