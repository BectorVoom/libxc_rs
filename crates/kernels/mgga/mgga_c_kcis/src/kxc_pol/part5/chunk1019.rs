//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1019/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1019<F: Float>(t6366: F, t949: F, t2986: F, t4740: F, t5250: F, t1226: F, t6428: F, t6406: F, t9825: F, t4764: F, t45: F, t6400: F, t13710: F, t13712: F, t13717: F, t13842: F, t18645: F, t18650: F, t18655: F, t18659: F, t18661: F, t18664: F, t18667: F, t18669: F, t18674: F, t18679: F, t18683: F, t9691: F, t9790: F) -> (F, F, F, F, F, F) {
    let t18997 = t6366 * t949;
    let t18999 = 6.0 * t2986 * t18997;
    let t19006 = t4740 * t5250;
    let t19011 = t6428 * t1226;
    let t19018 = t9825 * t6406;
    let t19019 = t19018 * t4764;
    let t19022 = t45 * t6400;
    let t19040 = -t9790 - 0.79148148148148148147e-2 * t9691 - 0.15829629629629629629e-1 * t13710 + 0.79148148148148148147e-2 * t13712 - t13842 + 0.23744444444444444444e-1 * t13717 + 0.39574074074074074073e-2 * t18645 - 0.19787037037037037037e-1 * t18650 + 0.71233333333333333332e-1 * t18655 - 0.47488888888888888888e-1 * t18659 - 0.11872222222222222222e-1 * t18661 - 0.10685e0 * t18664 + 0.14246666666666666666e0 * t18667 + 0.5936111111111111111e-2 * t18669 - 0.11872222222222222222e-1 * t18674 + 0.35616666666666666666e-1 * t18679 - 0.17808333333333333333e-1 * t18683;
    (t18999, t19006, t19011, t19019, t19022, t19040)
}
