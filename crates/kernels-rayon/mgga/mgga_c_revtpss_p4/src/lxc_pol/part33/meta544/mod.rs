//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta544 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1918;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1919;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta544(t1774: f64, t7627: f64, t7637: f64, t1294: f64, t8190: f64, t7652: f64, t1203: f64, t8201: f64, t1214: f64, t8208: f64, t2142: f64, t5219: f64, t1248: f64, t1287: f64, t1215: f64, t26922: f64, t26949: f64, t26994: f64, t29264: f64, t29268: f64, t29272: f64, t29275: f64, t5237: f64, t5429: f64, t5498: f64, t7602: f64, t7632: f64, t7636: f64, t7639: f64, t7643: f64, t7651: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t29278, t29279, t29283, t29287, t29293, t29297, t29301, t29304) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1918(t1774, t7627, t7637, t1294, t8190, t7652, t1203, t8201, t1214, t8208, t2142, t5219);
        let (t29308, t29311) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1919(t1248, t1287, t8208, t1215, t26922, t26949, t26994, t29264, t29268, t29272, t29275, t29279, t29283, t29287, t29293, t29297, t29301, t29304, t5237, t5429, t5498, t7602, t7632, t7636, t7639, t7643, t7651);
    (t29278, t29279, t29283, t29287, t29293, t29297, t29301, t29304, t29308, t29311)
}
