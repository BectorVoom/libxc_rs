//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1388/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1388<F: Float>(t22265: F, t22284: F, t22287: F, t22290: F, t22293: F, t27331: F, t27334: F, t27336: F, t27339: F, t27342: F, t27345: F, t27348: F, t22296: F, t27358: F, t27361: F, t27363: F, t27367: F, t27370: F, t27373: F, t27377: F, t27381: F, t27385: F, t27388: F, t27390: F) -> (F, F) {
    let t27887 = 0.3071625e0 * t27331 + 0.3071625e0 * t27334 + 0.15358125e0 * t27336 - 0.3560484375e1 * t27339 + 0.142419375e1 * t27342 + 0.1151859375e0 * t27345 - 0.76790625e-1 * t27348 - 0.32862666666666666666e0 * t22265 - 0.32862666666666666666e0 * t22284 - 0.65725333333333333332e0 * t22287 - 0.14605629629629629629e1 * t22290 + 0.10954222222222222222e1 * t22293;
    let t27900 = 0.10954222222222222222e1 * t22296 + 0.27385555555555555555e0 * t27358 - 0.65725333333333333333e0 * t27361 - 0.1898925e1 * t27363 + 0.49294e0 * t27367 - 0.32862666666666666666e0 * t27370 - 0.32862666666666666666e0 * t27373 + 0.24647e0 * t27377 + 0.49294e0 * t27381 + 0.24647e0 * t27385 + 0.5696775e1 * t27388 - 0.3071625e0 * t27390;
    (t27887, t27900)
}
