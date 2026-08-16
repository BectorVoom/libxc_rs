//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta201 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk811;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk812;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta201(t1264: f64, t5056: f64, t247: f64, t3629: f64, t5351: f64, t3626: f64, t3627: f64, t471: f64, t1715: f64, t1227: f64, t1261: f64, t1266: f64, t1808: f64, t3625: f64, t3647: f64, t3686: f64, t3705: f64, t5373: f64, t5379: f64, t5381: f64, t5384: f64, t5386: f64, t5391: f64, t1247: f64, t1252: f64, t1797: f64, t3708: f64, t3711: f64, t484: f64, t5254: f64, t5256: f64, t5258: f64, t5262: f64, t5266: f64, t5270: f64, t5274: f64, t5279: f64, t5287: f64, t5338: f64, t5372: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t5397, t5401, t5402, t5405, t5406, t5407, t5410) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk811(t1264, t5056, t247, t3629, t5351, t3626, t3627, t471, t1715, t1227, t1261, t1266, t1808, t3625, t3647, t3686, t3705, t5373, t5379, t5381, t5384, t5386, t5391);
        let t5412 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk812(t1247, t1252, t1261, t1797, t3708, t3711, t484, t5254, t5256, t5258, t5262, t5266, t5270, t5274, t5279, t5287, t5338, t5372, t5410);
    (t5397, t5401, t5402, t5405, t5406, t5407, t5412)
}
