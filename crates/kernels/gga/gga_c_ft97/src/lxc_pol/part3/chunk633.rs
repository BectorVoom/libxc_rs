//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 633/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk633<F: Float>(t1775: F, t3503: F, t3507: F, t3500: F, t3515: F, t1033: F, t8282: F, t3520: F, t11717: F, t3510: F, t12306: F, t12308: F, t12310: F, t12327: F, t12356: F, t12362: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12834 = 2.0 / 9.0 * t1775 * t3503;
    let t12836 = 4.0 / 9.0 * t1775 * t3507;
    let t12839 = 4.0 / 27.0 * t1775 * t3500;
    let t12850 = 2.0 / 9.0 * t1775 * t3515;
    let t12852 = t8282 * t1033;
    let t12864 = 4.0 / 3.0 * t1775 * t3520;
    let t12865 = t11717 * t3510;
    let t12889 = 2.0 / 27.0 * t12306;
    let t12890 = 4.0 / 27.0 * t12308;
    let t12891 = 4.0 / 81.0 * t12310;
    let t12897 = 2.0 / 27.0 * t12327;
    let t12911 = 4.0 / 9.0 * t12356;
    let t12913 = 4.0 / 81.0 * t12362;
    (t12834, t12836, t12839, t12850, t12852, t12864, t12865, t12889, t12890, t12891, t12897, t12911, t12913)
}
