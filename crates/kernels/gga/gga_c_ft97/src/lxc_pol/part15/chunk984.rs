//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 984/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk984<F: Float>(t52670: F, t65850: F, t65853: F, t65860: F, t65862: F, t88575: F, t88577: F, t88579: F, t88581: F, t88585: F, t88596: F, t4635: F, t4965: F) -> (F, F) {
    let t88598 = -0.40859909362962962964e0 * t88575 - 0.11652640818326474623e1 * t88577 + 0.37454916916049382717e0 * t88579 + 0.49939889221399176955e0 * t88581 - 0.23834947128395061728e0 * t88585 - 0.85124811172839506172e-2 * t65850 - 0.1134997482304526749e-1 * t65853 + 0.17024962234567901234e-1 * t65860 + 0.4539989929218106996e-1 * t65862 - 0.75666498820301783267e-1 * t52670 + 0.49523723477887517147e1 * t88596;
    let t88606 = t4965 * t4635;
    (t88598, t88606)
}
