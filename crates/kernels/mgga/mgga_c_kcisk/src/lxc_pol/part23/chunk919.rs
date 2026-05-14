//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 919/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk919<F: Float>(t1411: F, t19010: F, t3508: F, t5997: F, t12836: F, t12838: F, t12842: F, t18946: F, t18951: F, t18956: F, t18960: F, t18965: F, t18969: F, t18973: F, t18976: F, t18979: F, t18982: F, t18987: F, t18991: F, t18995: F, t19000: F, t19008: F) -> (F, F, F) {
    let t19011 = t1411 * t19010;
    let t19013 = t3508 * t5997;
    let t19014 = t1411 * t19013;
    let t19016 = -0.14739506172839506172e-1 * t18946 + 0.11054629629629629629e-2 * t18951 - 0.33163888888888888888e-2 * t18956 + 0.1621345679012345679e-1 * t18960 + 0.55273148148148148146e-2 * t18965 - 0.16581944444444444444e-2 * t18969 - 0.27636574074074074073e-2 * t18973 - 0.33163888888888888888e-2 * t18976 - 0.58958024691358024689e-2 * t18979 + 0.17687407407407407407e-1 * t18982 - 0.73697530864197530861e-3 * t18987 - 0.22109259259259259258e-2 * t18991 - 0.22109259259259259258e-2 * t18995 + 0.66327777777777777776e-2 * t19000 + 0.22109259259259259258e-2 * t12836 + 0.11054629629629629629e-2 * t12838 + 0.18424382716049382715e-2 * t12842 - 0.33163888888888888888e-2 * t19008 + 0.99491666666666666664e-2 * t19011 - 0.33163888888888888888e-2 * t19014;
    (t19011, t19014, t19016)
}
