//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 940/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk940<F: Float>(t1354: F, t25656: F, t1171: F, t7748: F, t1192: F, t19583: F, t2093: F, t3722: F, t7819: F, t5794: F, t21823: F, t5793: F, t1543: F, t7824: F, t5715: F, t5749: F) -> (F, F, F, F, F, F, F) {
    let t25657 = t1354 * t25656;
    let t25663 = t7748 * t1171;
    let t25665 = 1.0 * t25663 * t1192;
    let t25667 = 2.0 * t19583 * t2093;
    let t25668 = t3722 * t7819;
    let t25669 = t25668 * t5794;
    let t25672 = t5793 * t21823;
    let t25679 = t7824 * t1543;
    let t25683 = 2.0 * t5715 * t5749;
    (t25657, t25665, t25667, t25669, t25672, t25679, t25683)
}
