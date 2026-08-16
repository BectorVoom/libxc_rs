//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 402/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk402<F: Float>(t2919: F, t2921: F, t2849: F, t2853: F, t2860: F, t2882: F, t2890: F, t2892: F, t2895: F, t2896: F, t2903: F, t2914: F, t834: F, t839: F) -> (F, F) {
    let t2922 = t2919 * t2921;
    let t2925 = -t2849 - t2853 - t2860 + t2882 + t2890 + F::cast_from(0.24415406715670879921e-3_f64) * t834 * t2892 + F::cast_from(0.10843580882781524214e-1_f64) * t2895 * t2896 + F::cast_from(0.11696446794910408142e1_f64) * t839 * t2903 - F::cast_from(0.58482233974552040708e0_f64) * t839 * t2914 - F::cast_from(0.17315755899375863299e2_f64) * t839 * t2922;
    (t2922, t2925)
}
