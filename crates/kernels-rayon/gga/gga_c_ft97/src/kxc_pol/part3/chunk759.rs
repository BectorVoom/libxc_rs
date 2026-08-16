//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 759/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk759(t15839: f64, t419: f64, t11269: f64, t15737: f64, t11260: f64, t11265: f64, t11297: f64, t11299: f64, t11304: f64, t15837: f64, t8074: f64, t8079: f64, t8110: f64) -> (f64, f64, f64) {
    let t15840 = t419 * t15839;
    let t15842 = t11269 * t15737;
    let t15843 = t419 * t15842;
    let t15845 = 0.1134997482304526749e-1_f64 * t8074 + t8079 - 0.14187468528806584362e-2_f64 * t8110 - t11297 - 0.28374937057613168724e-2_f64 * t11299 + t11265 - 0.85124811172839506172e-2_f64 * t11260 + 0.85124811172839506172e-2_f64 * t11304 - 0.22699949646090534979e-1_f64 * t15837 + 0.28374937057613168724e-2_f64 * t15840 + 0.19862455940329218107e-1_f64 * t15843;
    (t15840, t15843, t15845)
}
