//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 831/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk831<F: Float>(t28: F, t34353: F, t32355: F, t6421: F, t25861: F, t5507: F, t32338: F, t3238: F, t7281: F, t7165: F, t965: F, t7243: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t34354 = t28 * t34353;
    let t34357 = t32355 * t6421;
    let t34358 = t28 * t34357;
    let t34361 = t5507 * t25861;
    let t34362 = t28 * t34361;
    let t34365 = t32338 * t6421;
    let t34366 = t28 * t34365;
    let t34368 = t3238 * t7281;
    let t34370 = t7165 * t965;
    let t34371 = t7243 * t34370;
    (t34354, t34357, t34358, t34361, t34362, t34365, t34366, t34368, t34370, t34371)
}
