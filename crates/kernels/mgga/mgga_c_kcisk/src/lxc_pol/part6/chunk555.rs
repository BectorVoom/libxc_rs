//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 555/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk555<F: Float>(t3626: F, t5668: F, t7738: F, t7742: F, t7746: F, t321: F, t2093: F, t5715: F, t2092: F, t1191: F, t3639: F, t2083: F) -> (F, F, F, F, F, F, F) {
    let t7748 = t3626 + F::new(0.11872222222222222222e-1) * t5668 - F::new(0.11872222222222222222e-1) * t7738 + F::new(0.35616666666666666666e-1) * t7742 - F::new(0.17808333333333333333e-1) * t7746;
    let t7750 = F::new(0.62182e-1) * t7748 * t321;
    let t7752 = F::new(2.0) * t5715 * t2093;
    let t7753 = t2092 * t2092;
    let t7754 = t7753 * t1191;
    let t7756 = F::new(2.0) * t3639 * t7754;
    let t7757 = t2083 * t2083;
    (t7748, t7750, t7752, t7753, t7754, t7756, t7757)
}
