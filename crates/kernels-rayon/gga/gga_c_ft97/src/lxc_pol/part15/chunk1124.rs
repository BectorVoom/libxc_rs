//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1124/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1124(t52670: f64, t65850: f64, t65853: f64, t65860: f64, t65862: f64, t88575: f64, t88577: f64, t88579: f64, t88581: f64, t88585: f64, t88596: f64, t4635: f64, t4965: f64) -> (f64, f64) {
    let t88598 = -0.40859909362962962964e0_f64 * t88575 - 0.11652640818326474623e1_f64 * t88577 + 0.37454916916049382717e0_f64 * t88579 + 0.49939889221399176955e0_f64 * t88581 - 0.23834947128395061728e0_f64 * t88585 - 0.85124811172839506172e-2_f64 * t65850 - 0.1134997482304526749e-1_f64 * t65853 + 0.17024962234567901234e-1_f64 * t65860 + 0.4539989929218106996e-1_f64 * t65862 - 0.75666498820301783267e-1_f64 * t52670 + 0.49523723477887517147e1_f64 * t88596;
    let t88606 = t4965 * t4635;
    (t88598, t88606)
}
