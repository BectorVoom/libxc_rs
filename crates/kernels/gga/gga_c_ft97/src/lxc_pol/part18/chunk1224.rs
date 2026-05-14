//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1224/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1224<F: Float>(t101858: F, t101864: F, t101869: F, t101873: F, t101886: F, t101891: F, t101896: F, t101902: F, t102256: F, t102257: F, t102258: F, t102261: F, t102095: F, t102106: F, t102113: F, t102126: F, t102134: F, t102146: F, t102159: F, t102169: F, t102184: F, t102196: F, t102206: F, t102217: F, t102227: F, t102237: F, t102251: F, t488: F) -> (F,) {
    let t102263 = -t101858 / 12.0 - t101864 / 3.0 + t101869 / 8.0 + 2.0 / 3.0 * t101873 + t102256 + t102257 - t102258 - 3.0 / 4.0 * t101886 - t101891 - 3.0 / 4.0 * t101896 - t102261 - 6.0 * t101902;
    let t102268 = t488 * (t102095 + t102106 + t102113 + t102126 + t102134 + t102146 + t102159 + t102169 + t102184 + t102196 + t102206 + t102217 + t102227 + t102237 + t102251 + t102263);
    (t102268,)
}
