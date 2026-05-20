//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta679 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2418;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2419;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta679<F: Float>(t3566: F, t3766: F, t5330: F, t12831: F, t12865: F, t1209: F, t13141: F, t17708: F, t371: F, t481: F, t482: F, t9291: F, t12627: F, t1284: F, t3624: F, t12640: F, t3555: F, t3781: F, t3617: F, t675: F, t1263: F, t215: F, t1122: F, t1261: F, t247: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t44551, t44561, t44578, t44607) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2418::<F>(t3566, t3766, t5330, t12831, t12865, t1209, t13141, t17708, t371, t481, t482, t9291);
        let (t44609, t44624, t44664, t44693, t44701, t44704) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2419::<F>(t12627, t1284, t3624, t12640, t3555, t3781, t5330, t3617, t675, t1263, t215, t1122, t1261, t247);
    (t44551, t44561, t44578, t44607, t44609, t44624, t44664, t44693, t44701, t44704)
}
