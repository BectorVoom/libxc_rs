//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1983/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1983<F: Float>(t213: F, t30247: F, t689: F, t6896: F, t7492: F, t102582: F, t102610: F, t102615: F, t102617: F, t102629: F, t1444: F, t1445: F, t30278: F, t7295: F, t8100: F, t94656: F, t96473: F, t96491: F, t96503: F, t96506: F, t96510: F, t96516: F, t98050: F) -> F {
    let t109706 = t213 * t30247;
    let t109715 = t689 * t7492 * t6896;
    let t109724 = F::cast_from(0.13009920719177044025e-2_f64) * t102582 - t96473 - F::cast_from(0.65854491829355115987e0_f64) * t109706 * t1445 + t96491 - F::cast_from(0.24093411633903331839e-3_f64) * t96503 + F::cast_from(0.24093411633903331839e-3_f64) * t96506 - F::cast_from(0.17135234354032049604e-2_f64) * t96510 + F::cast_from(0.8673628188205199462e0_f64) * t98050 * t8100 - F::cast_from(0.10975748638225852664e-1_f64) * t109715 - F::cast_from(0.4818682326780666368e-3_f64) * t102610 - F::cast_from(0.11565819519348392139e-2_f64) * t96516 + F::cast_from(0.10408353825846239354e2_f64) * t7295 * t94656 * t30278 * t1444 - t102615 + t102617 - F::cast_from(0.3427046870806409921e-2_f64) * t102629;
    t109724
}
