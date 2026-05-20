//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta267 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk994;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk995;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk996;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk997;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta267<F: Float>(t730: F, t9434: F, t2552: F, t722: F, t164: F, t172: F, t2555: F, t177: F, t9367: F, t9368: F, t9371: F, t701: F, t9275: F, t2582: F, t123: F, t173: F, t186: F, t2537: F, t2548: F, t2554: F, t2556: F, t2597: F, t2604: F, t729: F, t731: F, t739: F, t9291: F, t9394: F, t9485: F, t9488: F, t9501: F, t9508: F, t9514: F, t9517: F, t9521: F, t9524: F, t9484: F, t520: F, t512: F, t1450: F, t4135: F, t3850: F, t762: F, t749: F, t1331: F, t3857: F) -> (F, F, F, F, F, F, F) {
        let (t9525, t9530, t9533, t9536, t9537, t9540) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk994::<F>(t730, t9434, t2552, t722, t164, t172, t2555, t177, t9367, t9368, t9371, t701, t9275);
        let t9542 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk995::<F>(t2582, t9540);
        let t9543 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk996::<F>(t123, t173, t186, t2537, t2548, t2554, t2556, t2597, t2604, t729, t731, t739, t9291, t9394, t9485, t9488, t9501, t9508, t9514, t9517, t9521, t9524, t9525, t9530, t9533, t9536, t9537, t9542);
        let (t9544, t9546, t9547, t9552, t9555, t9559) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk997::<F>(t9484, t9543, t520, t512, t1450, t4135, t177, t3850, t762, t749, t1331, t3857);
    (t9542, t9544, t9546, t9547, t9552, t9555, t9559)
}
