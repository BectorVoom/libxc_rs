//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 572/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk572<F: Float>(t4108: F, t4115: F, t6020: F, t6066: F, t7914: F, t7917: F, t7920: F, t7932: F, t7939: F, t7945: F, t7947: F, t7951: F, t7954: F, t7957: F) -> F {
    let t7993 = -F::new(0.1294625e1) * t7932 + F::new(0.258925e1) * t7939 + t4108 + F::new(0.20128333333333333334e0) * t6020 - F::new(0.20128333333333333333e0) * t7914 + F::new(0.60385e0) * t7917 - F::new(0.301925e0) * t7920 + F::new(0.82524375e-1) * t7945 + F::new(0.16504875e0) * t7947 + t4115 + F::new(0.22076e0) * t6066 - F::new(0.5519e-1) * t7951 + F::new(0.33114e0) * t7954 - F::new(0.16557e0) * t7957;
    t7993
}
