//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 675/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk675<F: Float>(t601: F, t7780: F, t606: F, t1973: F, t1988: F, t1982: F, t1983: F, t361: F, t1980: F, t1979: F, t377: F) -> (F, F, F, F, F, F) {
    let t7781 = t7780 * t601;
    let t7782 = 0.45017719023973223821e-2 * t7781;
    let t7787 = t7780 * t606;
    let t7788 = 0.66040993808168719343e-2 * t7787;
    let t7789 = t1988 * t1973;
    let t7796 = t1982 * t361 * t1983;
    let t7797 = t1980 * t7796;
    let t7799 = t377 * t1979;
    (t7782, t7788, t7789, t7796, t7797, t7799)
}
