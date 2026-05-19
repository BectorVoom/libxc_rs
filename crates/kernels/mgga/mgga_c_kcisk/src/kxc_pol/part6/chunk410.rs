//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 410/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk410<F: Float>(t2885: F, t2888: F, t119: F, t56: F, t69: F, t45: F, t5: F, t157: F, t849: F, t52: F, t840: F, t846: F) -> (F, F, F, F, F, F) {
    let t2890 = F::cast_from(0.16081824322151104822e2_f64) * t2885 * t2888;
    let t2892 = t69 * t119 * t56;
    let t2895 = t45 * t5;
    let t2896 = t157 * t849;
    let t2899 = t840 * t52;
    let t2900 = F::new(1.0) / t2899;
    let t2901 = t846 * t846;
    (t2890, t2892, t2895, t2896, t2900, t2901)
}
