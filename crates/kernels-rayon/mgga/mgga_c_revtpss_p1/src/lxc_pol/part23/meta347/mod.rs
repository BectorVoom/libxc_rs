//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta347 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1650;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1651;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta347(t4424: f64, t72: f64, t686: f64, t2798: f64, t136: f64, t1559: f64, t2457: f64, t10535: f64, t10069: f64, t4496: f64, t1568: f64, t836: f64, t231: f64, t2783: f64, t2782: f64, t10867: f64, t225: f64, t213: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14519, t14520, t14522, t14523, t14524, t14525, t14533, t14535) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1650(t4424, t72, t686, t2798, t136, t1559, t2457, t10535, t10069, t4496, t1568, t836);
        let (t14537, t14539, t14545, t14546) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1651(t14535, t231, t2783, t2782, t10867, t225, t213);
    (t14519, t14520, t14522, t14523, t14524, t14525, t14533, t14537, t14539, t14545, t14546)
}
