//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1157/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1157<F: Float>(t1611: F, t1620: F, t21345: F, t22056: F, t2347: F, t2748: F, t32533: F, t33617: F, t33626: F, t33629: F, t33635: F, t33639: F, t33642: F, t33703: F, t33705: F, t33708: F, t33743: F, t33745: F, t4535: F, t6604: F, t9560: F, t9571: F) -> (F,) {
    let t33747 = -t1611 * t33743 - t1620 * t33745 + 2.0 * t21345 * t9560 - t22056 * t2748 - t2347 * t32533 + 2.0 * t33705 * t4535 + 2.0 * t33708 * t4535 - t6604 * t9571 - t33617 - t33626 - t33629 - t33635 + t33639 - t33642 + t33703;
    (t33747,)
}
