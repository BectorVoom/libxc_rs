//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta930 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3157;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3158;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta930(t12772: f64, t17673: f64, t3625: f64, t12910: f64, t12916: f64, t17460: f64, t17213: f64, t3172: f64, t5384: f64, t13069: f64, t5265: f64, t1260: f64, t17332: f64, t17747: f64, t17749: f64, t1222: f64, t16725: f64, t17471: f64, t16729: f64, t13017: f64, t5373: f64, t44546: f64, t5331: f64, t5334: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t57170, t57173, t57176, t57178, t57187) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3157(t12772, t17673, t3625, t12910, t12916, t17460, t17213, t3172, t5384, t13069, t5265, t1260, t17332);
        let (t57191, t57209, t57212, t57214, t57222) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3158(t12916, t17747, t17749, t1222, t16725, t17471, t16729, t13017, t5373, t44546, t5331, t5334);
    (t57170, t57173, t57176, t57178, t57187, t57191, t57209, t57212, t57214, t57222)
}
