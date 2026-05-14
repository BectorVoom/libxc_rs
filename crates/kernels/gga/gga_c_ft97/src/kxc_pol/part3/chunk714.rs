//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 714/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk714<F: Float>(t103: F, t16533: F, t16085: F, t16089: F, t16199: F, t16204: F, t16215: F, t16247: F, t16251: F, t16279: F, t16292: F, t16481: F, t16549: F, t18: F, t989: F, t15625: F) -> (F, F, F) {
    let t16550 = t16533 * t103;
    let t16562 = 2.0 * t16550 - 2.0 * t16247 - 4.0 * t16085 + 8.0 * t16279 - 4.0 * t16089 + 4.0 * t16251 - 12.0 * t16199 + 8.0 * t16204 - 2.0 * t16215 + 4.0 * t16292 - 2.0 * t16481;
    let t16563 = t16549 + t16562;
    let t16573 = t989 * t18;
    let t16579 = -t15625;
    (t16563, t16573, t16579)
}
