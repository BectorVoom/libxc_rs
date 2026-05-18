//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 871/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk871<F: Float>(t28312: F, t682: F, t2372: F, t8522: F, t11371: F, t15989: F, t22564: F, t22575: F, t22583: F, t28371: F, t28375: F, t28379: F, t28383: F, t28387: F, t28391: F, t28394: F, t28412: F, t28417: F) -> (F, F, F) {
    let t28539 = t682 * t28312;
    let t28546 = t2372 * t8522;
    let t28568 = F::new(0.14865e-1) * t28417 - F::new(0.2973e-1) * t28412 + F::new(0.1982e-1) * t28394 - t11371 - F::new(0.55033333333333333332e-2) * t15989 + F::new(0.27516666666666666666e-2) * t22564 - F::new(0.82549999999999999999e-2) * t22575 + F::new(0.41274999999999999999e-2) * t22583 - F::new(0.45861111111111111112e-2) * t28371 + F::new(0.1651e-1) * t28375 - F::new(0.82550000000000000001e-2) * t28379 - F::new(0.24765e-1) * t28383 + F::new(0.24765e-1) * t28387 - F::new(0.41275e-2) * t28391;
    (t28539, t28546, t28568)
}
