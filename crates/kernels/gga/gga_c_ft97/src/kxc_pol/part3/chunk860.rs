//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 860/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk860<F: Float>(t3483: F, t925: F, t13220: F, t11593: F, t13040: F, t13042: F, t13049: F, t13062: F, t13075: F, t13084: F, t17195: F, t17200: F, t17204: F, t17208: F, t17357: F, t17360: F, t17362: F, t17366: F, t1901: F, t446: F) -> F {
    let t17369 = t925 * t3483;
    let t17370 = t13220 * t17369;
    let t17373 = F::new(2.0) / F::new(9.0) * t1901 * t17195 + t1901 * t17200 / F::new(9.0) - F::new(10.0) / F::new(81.0) * t1901 * t17204 - F::new(8.0) / F::new(27.0) * t11593 * t17208 - t13040 - t13042 - t13049 + t13062 - t446 * t17357 / F::new(3.0) + t17360 / F::new(9.0) + F::new(2.0) / F::new(9.0) * t17362 + F::new(4.0) / F::new(27.0) * t13075 + t13084 - F::new(2.0) / F::new(9.0) * t1901 * t17366 - F::new(4.0) / F::new(9.0) * t1901 * t17370;
    t17373
}
