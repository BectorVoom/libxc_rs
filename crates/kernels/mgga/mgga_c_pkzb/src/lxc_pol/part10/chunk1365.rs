//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1365/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1365<F: Float>(t18427: F, t18430: F, t18468: F, t22230: F, t22233: F, t22236: F, t27262: F, t27289: F, t27295: F, t834: F, t18440: F, t18443: F, t18448: F, t27256: F, t27292: F, t841: F) -> (F, F, F) {
    let t27304 = t18468 - 56.0 / 27.0 * t18427 + 4.0 / 9.0 * t18430 - 56.0 / 27.0 * t22230 + 16.0 / 9.0 * t22233 - 2.0 / 3.0 * t22236 + 4.0 / 9.0 * t27295 - 2.0 / 3.0 * t27262 + t27289;
    let t27305 = t834 * t27304;
    let t27307 = 0.49671e0 * t27256 + t18440 - 0.18786444444444444445e1 * t18427 + 0.40256666666666666667e0 * t18430 + t18443 + 0.27595e0 * t18448 - 0.60385e0 * t27262 + 0.905775e0 * t27289 + 0.27595e0 * t27292 + 0.40256666666666666667e0 * t27295 + 0.258925e1 * t27305;
    let t27308 = t841 * t27304;
    (t27305, t27307, t27308)
}
