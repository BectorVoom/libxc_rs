//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 834/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk834<F: Float>(t10883: F, t13538: F, t13544: F, t14544: F, t14553: F, t18826: F, t18831: F, t18834: F, t18851: F, t3750: F, t4068: F, t688: F, t9558: F, t898: F, t900: F, t1268: F, t992: F) -> (F, F) {
    let t18852 = 0.1760655e0 * t18826 * t688 - 0.234754e0 * t4068 * t3750 - 0.117377e0 * t18831 * t688 + 0.234754e0 * t18834 - t10883 - 0.6419148148148148148e-1 * t9558 - 0.12838296296296296296e0 * t13538 + t14544 - t14553 - 0.19257444444444444444e0 * t13544 + t18851;
    let t18854 = t898 * t900 * t18852;
    let t18857 = t992 * t1268;
    (t18854, t18857)
}
