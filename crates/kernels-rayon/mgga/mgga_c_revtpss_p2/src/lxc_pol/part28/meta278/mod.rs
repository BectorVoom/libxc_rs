//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta278 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1244;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1245;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta278(t7301: f64, t7925: f64, t545: f64, t7910: f64, t2028: f64, t1904: f64, t2027: f64, t2030: f64, t213: f64, t561: f64, t7245: f64, t7248: f64, t7279: f64, t7288: f64, t7291: f64, t7295: f64, t7911: f64, t7917: f64, t7921: f64, t532: f64, t1450: f64, t2014: f64, t2034: f64, t5542: f64, t118: f64, t1502: f64, t1519: f64, t1843: f64, t1911: f64, t1932: f64, t2007: f64, t2011: f64, t508: f64, t569: f64, t651: f64, t6985: f64, t7725: f64, t7731: f64, t7734: f64, t7737: f64, t7744: f64, t7746: f64, t7883: f64, t7894: f64, t7899: f64, t7903: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7926, t7929, t7930, t7933) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1244(t7301, t7925, t545, t7910, t2028, t1904, t2027, t2030, t213, t561, t7245, t7248, t7279, t7288, t7291, t7295, t7911, t7917, t7921);
        let (t7934, t7935, t7937, t7939) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1245(t532, t7933, t1450, t2014, t2034, t5542, t118, t1502, t1519, t1843, t1911, t1932, t2007, t2011, t508, t569, t651, t6985, t7725, t7731, t7734, t7737, t7744, t7746, t7883, t7894, t7899, t7903);
    (t7926, t7929, t7930, t7933, t7934, t7935, t7937, t7939)
}
