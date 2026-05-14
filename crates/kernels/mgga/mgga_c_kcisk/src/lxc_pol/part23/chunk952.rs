//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 952/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk952<F: Float>(t3521: F, t5904: F, t5908: F, t5896: F, t12878: F, t12880: F, t1421: F, t19268: F, t19271: F, t19274: F, t19278: F, t19280: F, t19283: F, t19287: F, t19291: F, t19295: F, t19299: F, t19302: F, t19305: F, t19311: F, t19314: F, t19318: F, t5913: F) -> (F,) {
    let t19320 = 0.8760572888888888889e-3 * t3521 * t5904;
    let t19322 = 0.17521145777777777778e-2 * t3521 * t5908;
    let t19324 = 0.14600954814814814815e-2 * t3521 * t5896;
    let t19325 = 0.16426074166666666667e-2 * t1421 * t19268 - 0.21901432222222222221e-2 * t19271 - 0.7391733375e-3 * t1421 * t19274 + t19278 + 0.13140859333333333333e-2 * t1421 * t19280 + 0.39422577999999999999e-2 * t1421 * t19283 + 0.98556445e-3 * t1421 * t19287 + 0.39422578e-2 * t5913 * t19291 - 0.65704296666666666667e-3 * t1421 * t19295 - 0.26281718666666666666e-2 * t5913 * t19299 - 0.13140859333333333333e-2 * t1421 * t19302 - 0.52563437333333333332e-2 * t5913 * t19305 - 0.65704296666666666666e-3 * t12878 + 0.43802864444444444444e-3 * t12880 + 0.492782225e-3 * t1421 * t19311 - 0.65704296666666666666e-2 * t1421 * t19314 + t19318 - t19320 - t19322 + t19324;
    (t19325,)
}
