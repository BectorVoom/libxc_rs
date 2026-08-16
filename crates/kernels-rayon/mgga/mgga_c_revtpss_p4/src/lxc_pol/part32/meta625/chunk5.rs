//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1983/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1983(t213: f64, t30247: f64, t689: f64, t6896: f64, t7492: f64, t102582: f64, t102610: f64, t102615: f64, t102617: f64, t102629: f64, t1444: f64, t1445: f64, t30278: f64, t7295: f64, t8100: f64, t94656: f64, t96473: f64, t96491: f64, t96503: f64, t96506: f64, t96510: f64, t96516: f64, t98050: f64) -> f64 {
    let t109706 = t213 * t30247;
    let t109715 = t689 * t7492 * t6896;
    let t109724 = 0.13009920719177044025e-2_f64 * t102582 - t96473 - 0.65854491829355115987e0_f64 * t109706 * t1445 + t96491 - 0.24093411633903331839e-3_f64 * t96503 + 0.24093411633903331839e-3_f64 * t96506 - 0.17135234354032049604e-2_f64 * t96510 + 0.8673628188205199462e0_f64 * t98050 * t8100 - 0.10975748638225852664e-1_f64 * t109715 - 0.4818682326780666368e-3_f64 * t102610 - 0.11565819519348392139e-2_f64 * t96516 + 0.10408353825846239354e2_f64 * t7295 * t94656 * t30278 * t1444 - t102615 + t102617 - 0.3427046870806409921e-2_f64 * t102629;
    t109724
}
