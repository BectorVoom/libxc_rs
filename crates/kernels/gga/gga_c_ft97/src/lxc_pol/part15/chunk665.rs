//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 665/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk665<F: Float>(t120: F, t19993: F, t72: F, t8965: F, t920: F, t16854: F, t19977: F, t528: F, t126: F, t1005: F, t4466: F, t20049: F, t1631: F, t16853: F, t2014: F, t2021: F, t20589: F, t20592: F, t20596: F, t20599: F, t3359: F, t534: F, t7914: F, t8948: F, t8963: F, t8977: F, t8994: F) -> (F, F, F, F, F) {
    let t20603 = t72 * t19993 * t120;
    let t20606 = t8965 * t920;
    let t20607 = t16854 * t20606;
    let t20612 = t72 * t19977 * t528 * t120;
    let t20615 = t19977 * t126;
    let t20618 = t1005 * t4466;
    let t20623 = t20049 * t126;
    let t20630 = 0.17557713923258613e0 * t20589 * t120 - 0.35115427846517226e0 * t3359 * t20592 + 0.33205381699090447729e-3 * t8948 * t20596 + 0.23410285231011484e0 * t20599 * t120 - 0.79692916077817074549e-2 * t2014 * t20603 - t16853 - 0.8854768453090786061e-3 * t8963 * t20607 + 0.72343824494974941953e-3 * t2014 * t20612 - 0.5116527820486904976e-1 * t8977 * t20615 + 0.959348966341294683e-1 * t2021 * t20618 - 0.25159457085530922489e-1 * t7914 * t20615 - 0.532971647967385935e-1 * t534 * t20623 + 0.41932428475884870816e-1 * t1631 * t20618 - 0.91641760171536135284e-3 * t8994 * t20615;
    (t20603, t20606, t20607, t20612, t20630)
}
