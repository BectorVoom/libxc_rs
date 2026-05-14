//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 876/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk876<F: Float>(t34347: F, t30546: F, t8477: F, t1967: F, t8561: F, t30543: F, t8515: F, t10146: F, t420: F, t576: F, t1083: F, t137: F, t1511: F, t2020: F, t7440: F, t8631: F) -> (F, F, F, F, F, F, F, F) {
    let t34348 = 0.14291339372689912324e-3 * t34347;
    let t34349 = t30546 * t8477;
    let t34351 = t1967 * t8561;
    let t34352 = 0.37737710747524982482e-2 * t34351;
    let t34361 = t30543 * t8515;
    let t34362 = 0.12862205435420921092e-1 * t34361;
    let t34368 = t576 * t420 * t10146;
    let t34369 = t1083 * t137;
    let t34382 = t2020 * t1511;
    let t34383 = 7.0 / 144.0 * t34382;
    let t34390 = t7440 * t8631;
    (t34348, t34349, t34352, t34362, t34368, t34369, t34383, t34390)
}
