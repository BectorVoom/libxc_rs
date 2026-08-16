//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 839/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk839<F: Float>(t12831: F, t1341: F, t1340: F, t12827: F, t3748: F, t3751: F, t3755: F, t140: F, t3529: F, t3737: F, t3761: F, t11250: F, t461: F) -> (F, F, F, F, F) {
    let t12832 = t1341 * t12831;
    let t12833 = t1340 * t12832;
    let t12834 = t12827 * t12833;
    let t12836 = t3748 * t3751;
    let t12838 = t3748 * t3755;
    let t12841 = t140 * t3737 * t3529;
    let t12842 = t12841 * t3761;
    let t12845 = F::cast_from(0.29201909629629629629e-3_f64) * t11250 * t461;
    (t12834, t12836, t12838, t12842, t12845)
}
