//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta251 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1084;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta251(t11273: f64, t3160: f64, t2862: f64, t3128: f64, t1042: f64, t2853: f64, t3181: f64, t999: f64, t2866: f64, t914: f64, t936: f64, t2869: f64, t2919: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11277, t11280, t11281, t11285, t11286, t11289, t11291, t11293) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1084(t11273, t3160, t2862, t3128, t1042, t2853, t3181, t999, t2866, t914, t936, t2869, t2919);
    (t11277, t11280, t11281, t11285, t11286, t11289, t11291, t11293)
}
