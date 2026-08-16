//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta250 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1434;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1435;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta250(t9484: f64, t9543: f64, t520: f64, t512: f64, t1331: f64, t3857: f64, t2619: f64, t3825: f64, t1333: f64, t3863: f64, t2626: f64, t676: f64, t3869: f64, t2434: f64, t762: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9544, t9545, t9546, t9559, t9566, t9569, t9570, t9572) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1434(t9484, t9543, t520, t512, t1331, t3857, t2619, t3825, t1333, t3863, t2626, t676);
        let (t9574, t9575) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1435(t3869, t9572, t2434, t762);
    (t9544, t9545, t9546, t9559, t9566, t9569, t9570, t9572, t9574, t9575)
}
