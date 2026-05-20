//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3437/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3437<F: Float>(t4733: F, t64504: F, t981: F, t19049: F, t3034: F, t19045: F, t300: F, t983: F, t63940: F, t63943: F, t64327: F, t64329: F, t64488: F, t64491: F, t64493: F, t64496: F, t64498: F, t64500: F, t64503: F) -> (F, F, F, F) {
    let t64507 = F::cast_from(0.34631718211362927518e2_f64) * t981 * t64504 * t4733;
    let t64509 = F::cast_from(0.17315859105681463759e2_f64) * t19049 * t3034;
    let t64510 = t300 * t19045;
    let t64512 = F::cast_from(0.11696447245269292414e1_f64) * t64510 * t983;
    let t64513 = -t63940 - t63943 + t64488 - t64491 + t64493 + t64327 + t64496 - t64329 - t64498 + t64500 - t64503 - t64507 - t64509 - t64512;
    (t64507, t64509, t64512, t64513)
}
