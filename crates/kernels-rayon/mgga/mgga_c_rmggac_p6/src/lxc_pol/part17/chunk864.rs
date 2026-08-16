//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 864/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk864(t34592: f64, t38254: f64, t38255: f64, t38256: f64, t38257: f64, t7489: f64, t8574: f64, t9823: f64, t9828: f64, t9833: f64, t9837: f64, t38262: f64, t38263: f64, t38266: f64, t38267: f64, t38268: f64, t38269: f64, t38271: f64, t7537: f64, t9862: f64, t9866: f64, t9869: f64) -> (f64, f64) {
    let t44544 = -t9823 + t9828 + t9833 + t9837 + t7489 + t34592 - t38254 - t38255 - t38256 - t38257 + 0.25538759935978703639e-4_f64 * t8574;
    let t44548 = t38262 + t38263 - t38266 - t38267 - t38268 - t38269 - t9862 + t9866 - t7537 + t9869 + t38271;
    (t44544, t44548)
}
