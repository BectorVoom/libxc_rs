//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta482 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1832;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1833;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta482<F: Float>(t1043: F, t7152: F, t1089: F, t7135: F, t999: F, t7145: F, t1976: F, t3042: F, t988: F, t993: F, t378: F, t8521: F, t995: F, t7146: F, t342: F, t1097: F, t1983: F, t1986: F, t25461: F, t25466: F, t25470: F, t25473: F, t25476: F, t25480: F, t25484: F, t25487: F, t25588: F, t25591: F, t25593: F, t25597: F, t25601: F, t25605: F, t25607: F, t25611: F, t7144: F, t7147: F, t7151: F, t7153: F, t7159: F, t7162: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t25613, t25617, t25621, t25624, t25625, t25626, t25629) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1832::<F>(t1043, t7152, t1089, t7135, t999, t7145, t1976, t3042, t988, t993, t378, t8521, t995);
        let (t25631, t25634, t25637) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1833::<F>(t1043, t1089, t7146, t342, t7135, t1097, t1983, t1986, t25461, t25466, t25470, t25473, t25476, t25480, t25484, t25487, t25588, t25591, t25593, t25597, t25601, t25605, t25607, t25611, t25613, t25617, t25621, t25626, t25629, t7144, t7147, t7151, t7153, t7159, t7162);
    (t25613, t25617, t25621, t25624, t25625, t25626, t25629, t25631, t25634, t25637)
}
