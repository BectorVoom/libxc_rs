//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1275/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1275<F: Float>(t123925: F, t108158: F, t108161: F, t108172: F, t123909: F, t123914: F, t123919: F, t123923: F, t123929: F, t123933: F, t123937: F, t123965: F, t108211: F, t123941: F, t123945: F, t123949: F, t123952: F, t123955: F, t123959: F, t123962: F, t123968: F, t123972: F, t123975: F) -> (F, F) {
    let t124533 = t123925 / 3.0;
    let t124536 = -t123909 - t123914 - t108158 + 3.0 / 4.0 * t123919 + t123923 / 9.0 - t108161 - t108172 + t124533 - t123929 + 24.0 * t123933 + t123937 / 4.0;
    let t124543 = t123965 / 3.0;
    let t124547 = 2.0 * t123941 + t123945 / 3.0 + t108211 + t123949 / 6.0 + 8.0 / 3.0 * t123952 - 8.0 / 9.0 * t123955 - 4.0 / 3.0 * t123959 + t123962 - t124543 - 4.0 / 3.0 * t123968 + 2.0 * t123972 + 2.0 * t123975;
    (t124536, t124547)
}
