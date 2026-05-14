//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1157/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1157<F: Float>(t108639: F, t7006: F, t27574: F, t28662: F, t28660: F, t14763: F, t25069: F, t24330: F, t25112: F, t28671: F, t109127: F, t6256: F, t28680: F, t28654: F, t28677: F, t28652: F) -> (F, F, F, F, F, F, F, F) {
    let t111959 = t7006 * t108639;
    let t111967 = t27574 * t28662;
    let t111968 = t28660 * t111967;
    let t111989 = t14763 * t25069;
    let t112015 = 0.20003400327777777778e0 * t25112 * t24330 * t28671;
    let t112016 = t6256 * t109127;
    let t112018 = t28680 * t111967;
    let t112020 = t27574 * t28654;
    let t112021 = t28677 * t112020;
    let t112033 = t28652 * t112020;
    (t111959, t111968, t111989, t112015, t112016, t112018, t112021, t112033)
}
