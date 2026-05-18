//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1210/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1210<F: Float>(t37674: F, t37676: F, t37681: F, t37696: F, t37700: F, t39569: F, t39572: F, t39577: F, t39579: F, t39581: F, t39583: F, t39586: F) -> F {
    let t41456 = F::new(0.43663693315433241794e-2) * t39569 + F::new(0.26198215989259945076e-1) * t39572 - F::new(0.13869154784086829701e1) * t37674 + F::new(0.46230515946956099004e0) * t37676 - F::new(0.97574405393827830187e-2) * t37681 + F::new(0.10975748638225852664e0) * t39577 - F::new(0.51220160311720645767e0) * t39579 + F::new(0.34672886960217074252e0) * t39581 + F::new(0.10975748638225852664e0) * t39583 + F::new(0.86682217400542685632e-1) * t39586 + F::new(0.46574606203128791246e-1) * t37696 + F::new(0.23417857294518679246e0) * t37700;
    t41456
}
