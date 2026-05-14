//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 504/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk504<F: Float>(t2063: F, t4597: F, t1849: F, t1646: F, t2484: F, t2372: F, t4663: F, t1644: F, t2368: F, t4716: F, t2378: F, t827: F, t2386: F, t45: F, t2394: F, t4761: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6758 = t4597 * t2063;
    let t6763 = t1849 * t2063;
    let t6774 = t1646 * t2484;
    let t6777 = t4663 * t2372;
    let t6802 = t2368 * t1644;
    let t6817 = t4716 * t2372;
    let t6823 = t827 * t2378;
    let t6851 = t45 * t2386;
    let t6856 = t4761 * t2394;
    (t6758, t6763, t6774, t6777, t6802, t6817, t6823, t6851, t6856)
}
