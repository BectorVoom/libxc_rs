//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 455/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk455<F: Float>(t1212: F, t3696: F, t3697: F, t3571: F, t3657: F, t3573: F, t3577: F, t3581: F, t3585: F, t3607: F, t3609: F, t3652: F, t3654: F, t3659: F, t3663: F, t3666: F, t3669: F) -> (F, F) {
    let t3699 = t3696 * t3697 * t1212;
    let t3704 = 0.40256666666666666667e0 * t3571;
    let t3711 = 0.137975e0 * t3657;
    let t3716 = -0.1294625e1 * t3607 + 0.258925e1 * t3609 + t3704 + 0.20128333333333333334e0 * t3573 - 0.20128333333333333333e0 * t3577 + 0.60385e0 * t3581 - 0.301925e0 * t3585 + 0.82524375e-1 * t3652 + 0.16504875e0 * t3654 + t3711 + 0.11038e0 * t3659 - 0.27595e-1 * t3663 + 0.16557e0 * t3666 - 0.82785e-1 * t3669;
    (t3699, t3716)
}
