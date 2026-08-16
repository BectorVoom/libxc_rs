//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta424 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2035;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2036;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2037;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta424(t4424: f64, t72: f64, t686: f64, t2798: f64, t136: f64, t1559: f64, t2457: f64, t10535: f64, t10069: f64, t4496: f64, t1568: f64, t836: f64, t231: f64, t2783: f64, t2782: f64, t10519: f64, t10524: f64, t10943: f64, t14498: f64, t14502: f64, t14506: f64, t14507: f64, t14511: f64, t14512: f64, t14518: f64, t4366: f64, t4494: f64, t4504: f64, t4514: f64, t837: f64, t10867: f64, t225: f64, t213: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14519, t14520, t14522, t14523, t14524, t14525, t14533, t14535) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2035(t4424, t72, t686, t2798, t136, t1559, t2457, t10535, t10069, t4496, t1568, t836);
        let (t14537, t14539, t14540) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2036(t14535, t231, t2783, t2782, t10519, t10524, t10943, t14498, t14502, t14506, t14507, t14511, t14512, t14518, t14522, t14525, t14533, t4366, t4494, t4504, t4514, t837);
        let (t14545, t14546) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2037(t10867, t225, t213);
    (t14519, t14520, t14522, t14523, t14524, t14525, t14533, t14537, t14539, t14540, t14545, t14546)
}
