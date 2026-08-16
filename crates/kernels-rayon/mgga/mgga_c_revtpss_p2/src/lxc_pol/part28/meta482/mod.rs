//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta482 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1832;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1833;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta482(t1043: f64, t7152: f64, t1089: f64, t7135: f64, t999: f64, t7145: f64, t1976: f64, t3042: f64, t988: f64, t993: f64, t378: f64, t8521: f64, t995: f64, t7146: f64, t342: f64, t1097: f64, t1983: f64, t1986: f64, t25461: f64, t25466: f64, t25470: f64, t25473: f64, t25476: f64, t25480: f64, t25484: f64, t25487: f64, t25588: f64, t25591: f64, t25593: f64, t25597: f64, t25601: f64, t25605: f64, t25607: f64, t25611: f64, t7144: f64, t7147: f64, t7151: f64, t7153: f64, t7159: f64, t7162: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25613, t25617, t25621, t25624, t25625, t25626, t25629) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1832(t1043, t7152, t1089, t7135, t999, t7145, t1976, t3042, t988, t993, t378, t8521, t995);
        let (t25631, t25634, t25637) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1833(t1043, t1089, t7146, t342, t7135, t1097, t1983, t1986, t25461, t25466, t25470, t25473, t25476, t25480, t25484, t25487, t25588, t25591, t25593, t25597, t25601, t25605, t25607, t25611, t25613, t25617, t25621, t25626, t25629, t7144, t7147, t7151, t7153, t7159, t7162);
    (t25613, t25617, t25621, t25624, t25625, t25626, t25629, t25631, t25634, t25637)
}
