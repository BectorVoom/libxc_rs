//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 210/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk210<F: Float>(t135: F, t60: F, t4: F, t68: F, t85: F, t73: F, t2: F, t41: F, t74: F, t818: F, t71: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t854 = t135 * t135;
    let t855 = F::new(1.0) / t854;
    let t856 = t60 * t855;
    let t857 = t68 * t4;
    let t861 = t85 * t85;
    let t862 = F::new(1.0) / t861;
    let t863 = t73 * t862;
    let t866 = F::new(1.0) / t74 * t41 * t2;
    let t867 = t866 * t818;
    let t869 = t68 * t818;
    let t871 = f64::sqrt(t71);
    let t873 = t871 * t41 * t2;
    let t874 = t873 * t818;
    (t854, t855, t856, t857, t861, t862, t863, t866, t867, t869, t873, t874)
}
