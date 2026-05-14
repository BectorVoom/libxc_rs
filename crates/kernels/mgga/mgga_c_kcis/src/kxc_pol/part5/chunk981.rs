//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 981/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk981<F: Float>(t18527: F, t4557: F, t6700: F, t922: F, t3210: F, t3200: F, t1646: F, t1670: F, t829: F, t14322: F, t14316: F, t4633: F, t4606: F, t4670: F, t3293: F, t1035: F, t6316: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18528 = t18527 * t4557;
    let t18530 = t6700 * t922;
    let t18531 = t3210 * t18530;
    let t18532 = t3200 * t18531;
    let t18534 = t1646 * t1670;
    let t18535 = t18534 * t829;
    let t18536 = t14322 * t18535;
    let t18539 = t14316 * t4633;
    let t18542 = t4606 * t4670;
    let t18543 = t3293 * t18542;
    let t18546 = t1035 * t6316;
    (t18528, t18532, t18534, t18535, t18536, t18539, t18542, t18543, t18546)
}
