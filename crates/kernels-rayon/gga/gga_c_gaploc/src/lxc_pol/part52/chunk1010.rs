//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 1010/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk1010(t224: f64, t50308: f64, t50312: f64, t50478: f64, t50800: f64, t14443: f64, t44671: f64, t44674: f64, t44676: f64, t44678: f64, t44684: f64, t44687: f64, t44689: f64, t44692: f64, t44694: f64, t44705: f64, t45134: f64, t45148: f64, t45151: f64, t49820: f64, t49965: f64, t49968: f64, t49970: f64, t49972: f64, t856: f64) -> (f64, f64) {
    let t50803 = t224 * (t50308 + t50312 + t50478 + t50800);
    let t51228 = t14443 * t856 - t44671 - t44674 + t44676 - t44678 - t44684 + t44687 - t44689 + t44692 - t44694 - t44705 + t45134 + t45148 - t45151 - t49820 + t49965 + t49968 - t49970 + t49972;
    (t50803, t51228)
}
