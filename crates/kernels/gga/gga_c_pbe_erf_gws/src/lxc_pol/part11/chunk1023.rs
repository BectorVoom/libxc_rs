//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1023/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1023<F: Float>(t42452: F, t22084: F, t22590: F, t22592: F, t33523: F, t22594: F, t33527: F, t33530: F, t22599: F, t18527: F, t18529: F, t18556: F, t22606: F, t22609: F, t18562: F, t18567: F, t18571: F, t18574: F, t18577: F, t18580: F, t18587: F, t18594: F, t18599: F, t18604: F, t18607: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t48478 = 16.0 * t42452;
    let t48479 = 0.14035736153892489771e2 * t22084;
    let t48480 = 48.0 * t22590;
    let t48481 = 96.0 * t22592;
    let t48482 = 0.35089340384731224426e1 * t33523;
    let t48483 = 0.14035736153892489771e2 * t22594;
    let t48484 = 48.0 * t33527;
    let t48485 = 0.14649244029402527953e-2 * t33530;
    let t48486 = 0.22787712934626154593e-2 * t22599;
    let t48487 = -t48478 + t48479 + t18527 - t18529 - t48480 - t48481 - t48482 - t48483 - t48484 + t48485 - t48486 - t18556;
    let t48488 = 4.0 * t22606;
    let t48489 = 48.0 * t22609;
    let t48490 = -t18562 + t48488 + t18567 + t18571 - t18574 - t48489 + t18577 + t18580 + t18587 + t18594 + t18599 - t18604 - t18607;
    (t48478, t48479, t48480, t48481, t48482, t48483, t48484, t48485, t48486, t48487, t48488, t48489, t48490)
}
