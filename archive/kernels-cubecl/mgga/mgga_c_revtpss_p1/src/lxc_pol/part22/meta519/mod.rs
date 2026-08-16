//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta519 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2286;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2287;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2288;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2289;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta519<F: Float>(t141: F, t16903: F, t12254: F, t16715: F, t16708: F, t16710: F, t16712: F, t12296: F, t12297: F, t12299: F, t12301: F, t12303: F, t16706: F, t16717: F, t16722: F, t16727: F, t16731: F, t16735: F, t16740: F, t16744: F, t16748: F, t1139: F, t5095: F, t698: F, t1132: F, t12252: F, t12261: F, t12263: F, t12265: F, t12349: F, t12352: F, t16852: F, t16855: F, t16858: F, t16860: F, t16863: F, t16865: F, t16883: F, t16887: F, t16890: F, t16893: F, t16895: F, t16898: F, t16901: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16904, t16907, t16908, t16915, t16916, t16917, t16926) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2286::<F>(t141, t16903, t12254, t16715, t16708, t16710, t16712, t12296, t12297, t12299, t12301, t12303, t16706, t16717, t16722, t16727, t16731, t16735, t16740, t16744, t16748);
        let (t16927, t16929, t16931) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2287::<F>(t1139, t16926, t16710, t5095, t698);
        let (t16933, t16940) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2288::<F>(t1132, t16926, t16708, t16717, t16722, t16735, t16740, t16744, t16908, t16927, t16929, t16931);
        let t16942 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2289::<F>(t12252, t12261, t12263, t12265, t12349, t12352, t16731, t16852, t16855, t16858, t16860, t16863, t16865, t16883, t16887, t16890, t16893, t16895, t16898, t16901, t16904, t16940);
    (t16904, t16907, t16908, t16915, t16916, t16917, t16926, t16927, t16929, t16931, t16933, t16942)
}
