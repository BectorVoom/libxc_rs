//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 823/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk823<F: Float>(t25178: F, t2862: F, t6318: F, t24980: F, t25036: F, t25040: F, t25042: F, t25047: F, t25138: F, t25143: F, t25146: F, t25150: F, t25154: F, t25157: F, t25160: F, t25163: F, t25168: F, t25172: F, t25176: F) -> (F, F, F) {
    let t25180 = t2862 * t6318 * t25178;
    let t25181 = t24980 * t25180;
    let t25182 = -t25036 + 2.0 / 9.0 * t25040 - 2.0 / 9.0 * t25042 + 2.0 / 3.0 * t25047 - t25138 / 2.0 - 2.0 / 3.0 * t25143 + t25146 / 3.0 + t25150 / 3.0 - t25154 - t25157 - t25160 / 3.0 - t25163 / 9.0 + t25168 / 3.0 + t25172 / 6.0 + t25176 / 9.0 - t25181;
    (t25180, t25181, t25182)
}
