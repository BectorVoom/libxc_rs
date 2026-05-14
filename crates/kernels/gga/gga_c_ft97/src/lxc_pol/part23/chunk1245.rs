//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1245/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1245<F: Float>(t24437: F, t2574: F, t27787: F, t27814: F, t31036: F, t42500: F, t446: F, t713: F, t108186: F, t108187: F, t108188: F, t27775: F, t110089: F, t110090: F, t110095: F, t123909: F, t123914: F, t123919: F, t123923: F, t123925: F) -> (F, F, F, F) {
    let t123929 = t24437 * t2574 * t27787 * t27814;
    let t123933 = t446 * t42500 * t31036 * t713;
    let t123937 = t108186 * t108187 * t108188 * t27775;
    let t123939 = -t123909 / 3.0 - t123914 / 3.0 - t110089 + t123919 / 4.0 + t123923 / 27.0 - t110090 - t110095 + t123925 / 9.0 - t123929 / 3.0 + 8.0 * t123933 + t123937 / 12.0;
    (t123929, t123933, t123937, t123939)
}
