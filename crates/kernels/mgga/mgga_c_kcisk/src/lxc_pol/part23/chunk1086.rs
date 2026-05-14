//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1086/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1086<F: Float>(t3725: F, t5788: F, t1210: F, t3716: F, t6560: F, t12888: F, t2105: F, t3697: F, t14733: F, t14743: F, t14810: F, t19604: F, t19606: F, t19609: F, t19612: F, t19615: F, t19619: F, t19622: F, t19625: F, t4478: F, t6554: F, t6561: F) -> (F,) {
    let t21823 = t5788 * t3725;
    let t21824 = t21823 * t1210;
    let t21827 = t6560 * t3716;
    let t21830 = t2105 * t12888;
    let t21831 = t21830 * t3697;
    let t21838 = t19604 - t19606 + t19609 + t19612 + t19615 - t19619 - t19622 - t19625 + 0.34631511798751726598e2 * t4478 * t21824 + 0.17315755899375863299e2 * t4478 * t21827 + 0.1025389702100779493e4 * t14743 * t21831 - 0.23392893589820816284e1 * t14810 * t6554 + 0.34631511798751726598e2 * t14733 * t6561;
    (t21838,)
}
