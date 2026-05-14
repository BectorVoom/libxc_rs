//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 637/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk637<F: Float>(t173: F, t4479: F, t419: F, t11269: F, t15737: F, t11260: F, t11265: F, t11297: F, t11299: F, t11304: F, t15837: F, t8074: F, t8079: F, t8110: F, t15742: F, t3088: F) -> (F, F, F, F) {
    let t15839 = t173 * t4479;
    let t15840 = t419 * t15839;
    let t15842 = t11269 * t15737;
    let t15843 = t419 * t15842;
    let t15845 = 0.1134997482304526749e-1 * t8074 + t8079 - 0.14187468528806584362e-2 * t8110 - t11297 - 0.28374937057613168724e-2 * t11299 + t11265 - 0.85124811172839506172e-2 * t11260 + 0.85124811172839506172e-2 * t11304 - 0.22699949646090534979e-1 * t15837 + 0.28374937057613168724e-2 * t15840 + 0.19862455940329218107e-1 * t15843;
    let t15846 = t3088 * t15742;
    (t15840, t15843, t15845, t15846)
}
