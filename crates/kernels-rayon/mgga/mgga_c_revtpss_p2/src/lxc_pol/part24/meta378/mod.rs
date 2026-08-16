//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta378 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1272;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1273;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta378(t225: f64, t24698: f64, t480: f64, t1774: f64, t6622: f64, t1250: f64, t3720: f64, t6587: f64, t247: f64, t3719: f64, t12900: f64, t17629: f64, t21170: f64, t21189: f64, t21193: f64, t21216: f64, t21234: f64, t21249: f64, t24681: f64, t24684: f64, t3718: f64, t484: f64, t5381: f64, t5384: f64, t6683: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24699, t24700, t24704, t24705, t24706, t24713) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1272(t225, t24698, t480, t1774, t6622, t1250, t3720, t6587);
        let (t24715, t24722) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1273(t247, t24713, t3719, t12900, t17629, t21170, t21189, t21193, t21216, t21234, t21249, t24681, t24684, t24700, t24706, t3718, t484, t5381, t5384, t6683);
    (t24699, t24700, t24704, t24705, t24706, t24713, t24715, t24722)
}
