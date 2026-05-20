//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta562 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2007;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2008;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta562<F: Float>(t857: F, t93066: F, t25222: F, t2656: F, t2482: F, t596: F, t7043: F, t2677: F, t10741: F, t25234: F, t10709: F, t25227: F, t2661: F, t240: F, t25260: F, t10728: F, t2479: F, t25228: F, t9775: F, t10732: F, t10705: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t93067, t93069, t93072, t93073, t93077, t93080) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2007::<F>(t857, t93066, t25222, t2656, t2482, t596, t7043, t2677, t10741, t25234, t10709, t25227, t2661);
        let (t93082, t93084, t93086, t93088, t93091, t93095) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2008::<F>(t240, t25260, t10728, t2661, t2479, t25222, t25228, t9775, t10732, t25227, t10705, t25234);
    (t93067, t93069, t93072, t93073, t93077, t93080, t93082, t93084, t93086, t93088, t93091, t93095)
}
