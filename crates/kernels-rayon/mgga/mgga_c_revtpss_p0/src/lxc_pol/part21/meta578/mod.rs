//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta578 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2285;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2286;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta578(t17879: f64, t460: f64, t3584: f64, t5457: f64, t5351: f64, t1269: f64, t3766: f64, t1280: f64, t17345: f64, t1287: f64, t17389: f64, t17600: f64, t1248: f64, t5412: f64, t1204: f64, t12723: f64, t1281: f64, t1285: f64, t1288: f64, t12987: f64, t17289: f64, t17307: f64, t17861: f64, t17864: f64, t17869: f64, t17876: f64, t1825: f64, t3552: f64, t3666: f64, t3751: f64, t3755: f64, t3782: f64, t5449: f64, t5459: f64, t5466: f64, t5478: f64, t5481: f64, t5494: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17880, t17883, t17884, t17887, t17888, t17893, t17902, t17905) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2285(t17879, t460, t3584, t5457, t5351, t1269, t3766, t1280, t17345, t1287, t17389, t17600);
        let (t17909, t17912) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2286(t1248, t1287, t5412, t1204, t12723, t1281, t1285, t1288, t12987, t17289, t17307, t17861, t17864, t17869, t17876, t17880, t17884, t17888, t17893, t17902, t17905, t1825, t3552, t3666, t3751, t3755, t3782, t5449, t5459, t5466, t5478, t5481, t5494);
    (t17880, t17883, t17884, t17887, t17888, t17893, t17902, t17905, t17909, t17912)
}
