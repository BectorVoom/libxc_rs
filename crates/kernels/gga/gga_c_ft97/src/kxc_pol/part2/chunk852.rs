//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 852/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk852<F: Float>(t4167: F, t684: F, t10703: F, t10514: F, t15256: F, t15260: F, t15263: F, t15267: F, t15271: F, t15273: F, t15274: F, t15277: F, t15281: F, t15286: F, t15291: F, t15296: F, t15300: F, t1901: F, t3281: F, t446: F) -> (F,) {
    let t15303 = t4167 * t684;
    let t15304 = t10703 * t15303;
    let t15307 = -4.0 / 9.0 * t1901 * t15256 - 2.0 / 9.0 * t1901 * t15260 + 2.0 / 3.0 * t446 * t15263 + 2.0 / 9.0 * t3281 * t15267 + t15271 + t15273 - t446 * t15274 / 3.0 + 4.0 / 3.0 * t446 * t15277 + 2.0 / 3.0 * t446 * t15281 + 2.0 / 3.0 * t446 * t15286 + 8.0 / 27.0 * t10514 + 4.0 / 27.0 * t1901 * t15291 + 4.0 / 27.0 * t1901 * t15296 - 4.0 / 9.0 * t1901 * t15300 - 2.0 / 9.0 * t1901 * t15304;
    (t15307,)
}
