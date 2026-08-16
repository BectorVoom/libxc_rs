//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta519 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2286;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2287;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2288;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2289;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta519(t141: f64, t16903: f64, t12254: f64, t16715: f64, t16708: f64, t16710: f64, t16712: f64, t12296: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t16706: f64, t16717: f64, t16722: f64, t16727: f64, t16731: f64, t16735: f64, t16740: f64, t16744: f64, t16748: f64, t1139: f64, t5095: f64, t698: f64, t1132: f64, t12252: f64, t12261: f64, t12263: f64, t12265: f64, t12349: f64, t12352: f64, t16852: f64, t16855: f64, t16858: f64, t16860: f64, t16863: f64, t16865: f64, t16883: f64, t16887: f64, t16890: f64, t16893: f64, t16895: f64, t16898: f64, t16901: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16904, t16907, t16908, t16915, t16916, t16917, t16926) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2286(t141, t16903, t12254, t16715, t16708, t16710, t16712, t12296, t12297, t12299, t12301, t12303, t16706, t16717, t16722, t16727, t16731, t16735, t16740, t16744, t16748);
        let (t16927, t16929, t16931) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2287(t1139, t16926, t16710, t5095, t698);
        let (t16933, t16940) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2288(t1132, t16926, t16708, t16717, t16722, t16735, t16740, t16744, t16908, t16927, t16929, t16931);
        let t16942 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2289(t12252, t12261, t12263, t12265, t12349, t12352, t16731, t16852, t16855, t16858, t16860, t16863, t16865, t16883, t16887, t16890, t16893, t16895, t16898, t16901, t16904, t16940);
    (t16904, t16907, t16908, t16915, t16916, t16917, t16926, t16927, t16929, t16931, t16933, t16942)
}
