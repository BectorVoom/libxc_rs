//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1257/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1257<F: Float>(t108345: F, t110159: F, t110160: F, t110169: F, t110182: F, t124127: F, t124133: F, t124137: F, t124141: F, t124144: F, t124148: F, t97061: F, t1424: F, t18139: F, t1434: F, t193: F, t2506: F) -> (F, F, F) {
    let t124150 = 2.0 / 3.0 * t124127 + 2.0 / 27.0 * t108345 + t124133 / 9.0 - 4.0 * t124137 + 4.0 / 3.0 * t124141 - 2.0 / 9.0 * t124144 + 8.0 / 27.0 * t97061 - t110159 - t110160 + t110169 - 4.0 / 9.0 * t124148 + t110182;
    let t124151 = t1424 * t18139;
    let t124154 = t1434 * t193 * t2506 * t124151;
    (t124150, t124151, t124154)
}
