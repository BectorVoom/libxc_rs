//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 864/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk864<F: Float>(t34592: F, t38254: F, t38255: F, t38256: F, t38257: F, t7489: F, t8574: F, t9823: F, t9828: F, t9833: F, t9837: F, t38262: F, t38263: F, t38266: F, t38267: F, t38268: F, t38269: F, t38271: F, t7537: F, t9862: F, t9866: F, t9869: F) -> (F, F) {
    let t44544 = -t9823 + t9828 + t9833 + t9837 + t7489 + t34592 - t38254 - t38255 - t38256 - t38257 + F::new(0.25538759935978703639e-4) * t8574;
    let t44548 = t38262 + t38263 - t38266 - t38267 - t38268 - t38269 - t9862 + t9866 - t7537 + t9869 + t38271;
    (t44544, t44548)
}
