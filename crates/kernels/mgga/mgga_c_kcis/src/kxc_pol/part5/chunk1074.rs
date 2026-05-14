//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1074/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1074<F: Float>(t3393: F, t6661: F, t6665: F, t1154: F, t167: F, t5153: F, t10541: F, t15008: F, t19972: F, t19974: F, t19977: F, t19980: F, t19983: F, t19986: F, t19989: F, t19992: F, t19996: F, t20000: F, t20003: F, t20008: F, t2429: F, t368: F, t5133: F, t86: F) -> (F,) {
    let t20010 = t3393 * t6661;
    let t20012 = t3393 * t6665;
    let t20015 = t1154 * t5153 * t167;
    let t20018 = -t10541 + 0.35374814814814814815e-1 * t19972 - 0.15918666666666666667e0 * t5133 * t19974 + 0.26531111111111111111e0 * t5133 * t19977 - 0.11791604938271604938e0 * t5133 * t19980 - 0.17687407407407407407e0 * t15008 * t19983 + 0.21224888888888888889e0 * t15008 * t19986 + 0.53062222222222222222e-1 * t5133 * t19989 - 0.44218518518518518518e-1 * t5133 * t19992 + 0.10612444444444444444e0 * t5133 * t19996 - 0.88437037037037037037e-1 * t5133 * t20000 - 0.39796666666666666666e-1 * t86 * t368 * t20003 - 0.26531111111111111111e-1 * t20008 - 0.29479012345679012345e-1 * t20010 - 0.35374814814814814815e-1 * t20012 - 0.10612444444444444444e0 * t2429 * t20015;
    (t20018,)
}
