//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta614 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2145;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2146;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2147;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2148;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta614(t25207: f64, t98651: f64, t1468: f64, t2411: f64, t14365: f64, t1544: f64, t2257: f64, t198: f64, t205: f64, t7086: f64, t4433: f64, t890: f64, t1940: f64, t1963: f64, t2403: f64, t25198: f64, t25206: f64, t25208: f64, t25449: f64, t27158: f64, t27160: f64, t27169: f64, t27364: f64, t27368: f64, t27395: f64, t4541: f64, t605: f64, t7087: f64, t7783: f64, t98627: f64, t98635: f64, t98637: f64, t98650: f64, t2255: f64, t27383: f64, t61155: f64, t27375: f64, t92790: f64, t14767: f64, t27159: f64, t4537: f64, t15071: f64, t30: f64, t61203: f64, t892: f64, t14749: f64, t7188: f64, t11064: f64, t7782: f64, t25436: f64, t25446: f64, t25452: f64, t27173: f64, t27385: f64, t51780: f64, t7091: f64, t7750: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t98652, t98659, t98662, t98669, t98674) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2145(t25207, t98651, t1468, t2411, t14365, t1544, t2257, t198, t205, t7086, t4433, t890);
        let t98678 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2146(t25207, t98674, t1940, t1963, t2403, t25198, t25206, t25208, t25449, t27158, t27160, t27169, t27364, t27368, t27395, t4541, t605, t7087, t7783, t98627, t98635, t98637, t98650, t98652, t98659, t98662, t98669);
        let (t98684, t98688, t98694, t98699, t98702, t98705) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2147(t1940, t2255, t7087, t27383, t61155, t27375, t92790, t14767, t27159, t4537, t605, t15071, t30);
        let (t98719, t98722, t98725) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2148(t25207, t61203, t4433, t605, t892, t14749, t27159, t198, t7188, t11064, t7782, t1468, t1940, t2403, t25206, t25436, t25446, t25452, t27158, t27173, t27368, t27385, t51780, t7087, t7091, t7750, t98684, t98688, t98694, t98699, t98702, t98705);
    (t98669, t98674, t98678, t98684, t98719, t98722, t98725)
}
