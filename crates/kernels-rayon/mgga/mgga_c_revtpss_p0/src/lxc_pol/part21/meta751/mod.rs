//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta751 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2628;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2629;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta751(t47133: f64, t47135: f64, t13665: f64, t9572: f64, t1320: f64, t13680: f64, t47145: f64, t47147: f64, t47149: f64, t3863: f64, t5569: f64, t3860: f64, t5571: f64, t9419: f64, t40076: f64, t40079: f64, t47131: f64, t47138: f64, t47140: f64, t47142: f64, t47152: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48322, t48323, t48325, t48327, t48328, t48329, t48330, t48332, t48333) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2628(t47133, t47135, t13665, t9572, t1320, t13680, t47145, t47147, t47149, t3863, t5569, t3860);
        let (t48334, t48336, t48337) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2629(t48333, t5571, t9419, t40076, t40079, t47131, t47138, t47140, t47142, t47152, t48322, t48323, t48325, t48327, t48328, t48329, t48330, t48332);
    (t48322, t48323, t48325, t48327, t48328, t48329, t48330, t48332, t48334, t48336, t48337)
}
