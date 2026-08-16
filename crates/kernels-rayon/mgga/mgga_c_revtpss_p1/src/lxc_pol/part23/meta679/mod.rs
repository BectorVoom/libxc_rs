//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta679 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2418;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2419;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta679(t3566: f64, t3766: f64, t5330: f64, t12831: f64, t12865: f64, t1209: f64, t13141: f64, t17708: f64, t371: f64, t481: f64, t482: f64, t9291: f64, t12627: f64, t1284: f64, t3624: f64, t12640: f64, t3555: f64, t3781: f64, t3617: f64, t675: f64, t1263: f64, t215: f64, t1122: f64, t1261: f64, t247: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44551, t44561, t44578, t44607) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2418(t3566, t3766, t5330, t12831, t12865, t1209, t13141, t17708, t371, t481, t482, t9291);
        let (t44609, t44624, t44664, t44693, t44701, t44704) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2419(t12627, t1284, t3624, t12640, t3555, t3781, t5330, t3617, t675, t1263, t215, t1122, t1261, t247);
    (t44551, t44561, t44578, t44607, t44609, t44624, t44664, t44693, t44701, t44704)
}
