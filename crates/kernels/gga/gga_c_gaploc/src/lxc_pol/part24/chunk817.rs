//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 817/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk817<F: Float>(t313: F, t8637: F, t723: F, t8528: F, t1445: F, t1710: F, t2949: F, t3031: F, t4614: F, t2950: F, t4673: F, t1035: F, t2066: F, t3049: F, t773: F, t1022: F, t701: F) -> (F, F, F, F, F, F, F, F) {
    let t8638 = t313 * t8637;
    let t8645 = t8528 * t723;
    let t8646 = t1445 * t8645;
    let t8649 = t2949 * t1710;
    let t8650 = t1445 * t8649;
    let t8655 = t4614 * t3031;
    let t8658 = t4673 * t2950;
    let t8663 = t2066 * t1035;
    let t8666 = t773 * t3049;
    let t8669 = t1022 * t701;
    (t8638, t8646, t8650, t8655, t8658, t8663, t8666, t8669)
}
