//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 503/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk503<F: Float>(t2477: F, t696: F, t1814: F, t2063: F, t1806: F, t2488: F, t2487: F, t5101: F, t2365: F, t821: F) -> (F, F, F, F, F) {
    let t6729 = t696 * t2477;
    let t6734 = t1814 * t2063;
    let t6741 = t1806 * t2488;
    let t6746 = t5101 * t2487;
    let t6756 = t821 * t2365;
    (t6729, t6734, t6741, t6746, t6756)
}
