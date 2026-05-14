//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1368/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1368<F: Float>(t114965: F, t118671: F, t118674: F, t118677: F, t118680: F, t118682: F, t119709: F, t119712: F, t119875: F, t119877: F, t119879: F, t119926: F, t119946: F, t119966: F, t119986: F, t15087: F, t15094: F, t1611: F, t21345: F, t2347: F, t33705: F, t33745: F, t34906: F, t34909: F, t34912: F, t4530: F, t6638: F, t9882: F) -> (F,) {
    let t119998 = -t118671 + t118674 + 4.0 * t15087 * t34909 - 2.0 * t33745 * t6638 - t118677 + 2.0 * t15087 * t34912 - t1611 * (t119926 + t119946 + t119966 + t119986) - 2.0 * t114965 * t2347 - t118680 + t118682 - t119709 - t119712 + 4.0 * t21345 * t33705 - 12.0 * t15094 * t9882 * t6638 + t119875 - t119877 + t119879 - t4530 * t34906;
    (t119998,)
}
