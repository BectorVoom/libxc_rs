//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta284 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1266;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1267;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1268;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1269;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta284(t730: f64, t9434: f64, t2552: f64, t722: f64, t164: f64, t172: f64, t2555: f64, t177: f64, t9367: f64, t9368: f64, t9371: f64, t701: f64, t9275: f64, t2582: f64, t123: f64, t173: f64, t186: f64, t2537: f64, t2548: f64, t2554: f64, t2556: f64, t2597: f64, t2604: f64, t729: f64, t731: f64, t739: f64, t9291: f64, t9394: f64, t9485: f64, t9488: f64, t9501: f64, t9508: f64, t9514: f64, t9517: f64, t9521: f64, t9524: f64, t9484: f64, t520: f64, t512: f64, t1331: f64, t3857: f64, t2619: f64, t3825: f64, t1333: f64, t3863: f64, t2626: f64, t676: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9525, t9530, t9533, t9536, t9537, t9540) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1266(t730, t9434, t2552, t722, t164, t172, t2555, t177, t9367, t9368, t9371, t701, t9275);
        let t9542 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1267(t2582, t9540);
        let t9543 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1268(t123, t173, t186, t2537, t2548, t2554, t2556, t2597, t2604, t729, t731, t739, t9291, t9394, t9485, t9488, t9501, t9508, t9514, t9517, t9521, t9524, t9525, t9530, t9533, t9536, t9537, t9542);
        let (t9544, t9546, t9559, t9566, t9569, t9570, t9572) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1269(t9484, t9543, t520, t512, t1331, t3857, t2619, t3825, t1333, t3863, t2626, t676);
    (t9542, t9544, t9546, t9559, t9566, t9569, t9570, t9572)
}
