//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 937/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk937<F: Float>(t13544: F, t13550: F, t13559: F, t13569: F, t14532: F, t14554: F, t2417: F, t4068: F, t688: F, t9558: F, t9560: F, t9562: F, t9564: F) -> F {
    let t14555 = -F::new(0.9628722222222222222e-1) * t9562 - F::new(0.10591594444444444444e1) * t13544 + F::new(0.28886166666666666666e0) * t13569 + F::new(0.57772333333333333332e0) * t13550 - F::new(0.86658499999999999998e0) * t13559 - F::new(0.234754e0) * t14532 * t688 - F::new(0.117377e0) * t4068 * t2417 - F::new(0.12838296296296296296e0) * t9558 + F::new(0.4814361111111111111e-1) * t9564 + F::new(0.3209574074074074074e-1) * t9560 + t14554;
    t14555
}
