//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1357/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1357<F: Float>(t3512: F, t8279: F, t1340: F, t27118: F, t5606: F, t6363: F, t27396: F, t488: F, t32287: F, t34866: F, t27104: F, t9497: F, t32255: F, t8244: F, t394: F, t8232: F) -> (F, F, F, F, F, F, F, F) {
    let t119813 = t3512 * t8279;
    let t119815 = t1340 * t27118;
    let t119817 = t5606 * t6363;
    let t119819 = t27396 * t488;
    let t119821 = t32287 * t34866;
    let t119823 = t9497 * t27104;
    let t119825 = t32255 * t8244;
    let t119827 = t8232 * t394;
    (t119813, t119815, t119817, t119819, t119821, t119823, t119825, t119827)
}
