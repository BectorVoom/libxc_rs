//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 572/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk572<F: Float>(t4108: F, t4115: F, t6020: F, t6066: F, t7914: F, t7917: F, t7920: F, t7932: F, t7939: F, t7945: F, t7947: F, t7951: F, t7954: F, t7957: F) -> F {
    let t7993 = -F::cast_from(0.1294625e1_f64) * t7932 + F::cast_from(0.258925e1_f64) * t7939 + t4108 + F::cast_from(0.20128333333333333334e0_f64) * t6020 - F::cast_from(0.20128333333333333333e0_f64) * t7914 + F::cast_from(0.60385e0_f64) * t7917 - F::cast_from(0.301925e0_f64) * t7920 + F::cast_from(0.82524375e-1_f64) * t7945 + F::cast_from(0.16504875e0_f64) * t7947 + t4115 + F::cast_from(0.22076e0_f64) * t6066 - F::cast_from(0.5519e-1_f64) * t7951 + F::cast_from(0.33114e0_f64) * t7954 - F::cast_from(0.16557e0_f64) * t7957;
    t7993
}
