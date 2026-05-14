//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 815/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk815<F: Float>(t13542: F, t10883: F, t13538: F, t13547: F, t13553: F, t13556: F, t13562: F, t13565: F, t14541: F, t14544: F, t14550: F, t2380: F, t13544: F, t13550: F, t13559: F, t13569: F, t14532: F, t2417: F, t4068: F, t688: F, t9558: F, t9560: F, t9562: F, t9564: F) -> (F,) {
    let t14553 = 0.19257444444444444444e0 * t13542;
    let t14554 = 0.1760655e0 * t14541 * t2380 + t14544 - 0.9628722222222222222e-1 * t13556 - 0.1604787037037037037e0 * t13547 + 0.38514888888888888888e0 * t13553 + 0.28886166666666666666e0 * t13565 - 0.11554466666666666666e1 * t13562 + 0.234754e0 * t14550 - t10883 - 0.6419148148148148148e-1 * t13538 - t14553;
    let t14555 = -0.9628722222222222222e-1 * t9562 - 0.10591594444444444444e1 * t13544 + 0.28886166666666666666e0 * t13569 + 0.57772333333333333332e0 * t13550 - 0.86658499999999999998e0 * t13559 - 0.234754e0 * t14532 * t688 - 0.117377e0 * t4068 * t2417 - 0.12838296296296296296e0 * t9558 + 0.4814361111111111111e-1 * t9564 + 0.3209574074074074074e-1 * t9560 + t14554;
    (t14555,)
}
