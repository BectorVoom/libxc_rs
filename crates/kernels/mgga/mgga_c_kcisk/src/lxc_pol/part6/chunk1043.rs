//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1043/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1043<F: Float>(t12845: F, t1421: F, t26577: F, t26579: F, t26600: F, t26602: F, t31009: F, t31013: F, t31017: F, t31021: F, t31025: F, t31060: F, t31097: F, t31131: F, t456: F) -> F {
    let t31133 = -F::new(0.26281718666666666667e-2) * t26577 + F::new(0.21901432222222222222e-2) * t26579 - F::new(0.59133867e-2) * t26600 + F::new(0.13140859333333333334e-2) * t26602 + t12845 + F::new(0.1478346675e-2) * t1421 * t31009 - F::new(0.59133867e-2) * t1421 * t31013 + F::new(0.39422577999999999999e-2) * t1421 * t31017 + F::new(0.39422577999999999999e-2) * t1421 * t31021 - F::new(0.36958666875e-3) * t456 * t31025 + t31060 + t31097 + t31131;
    t31133
}
