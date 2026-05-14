//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 534/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk534<F: Float>(t1450: F, t7831: F, t1340: F, t1411: F, t2232: F, t5886: F, t2236: F, t5606: F, t3530: F, t3533: F, t7706: F, t2075: F, t2083: F, t3539: F, t2191: F, t3544: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7832 = t1450 * t7831;
    let t7833 = t1340 * t7832;
    let t7834 = t1411 * t7833;
    let t7836 = t5886 * t2232;
    let t7837 = t1411 * t7836;
    let t7839 = t5606 * t2236;
    let t7840 = t1411 * t7839;
    let t7846 = t3530 * t3533 * t7706;
    let t7850 = t3539 * t2075 * t2083;
    let t7853 = t2075 * t2191;
    let t7854 = t3544 * t7853;
    (t7832, t7833, t7834, t7836, t7837, t7839, t7840, t7846, t7850, t7854)
}
