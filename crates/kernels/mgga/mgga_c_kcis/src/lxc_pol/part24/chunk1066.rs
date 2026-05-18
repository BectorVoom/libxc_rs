//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1066/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1066<F: Float>(t5189: F, t7740: F, t14665: F, t2189: F, t14668: F, t7743: F, t5036: F, t7766: F, t10491: F, t8064: F, t1203: F, t10498: F) -> (F, F, F, F, F, F, F) {
    let t27992 = t7740 * t5189;
    let t27993 = t14665 * t2189;
    let t27995 = F::new(2.0) * t14668 * t7743;
    let t27996 = t5036 * t7766;
    let t27998 = F::new(2.0) * t10491 * t8064;
    let t27999 = t8064 * t1203;
    let t28001 = F::new(6.0) * t10498 * t27999;
    (t27992, t27993, t27995, t27996, t27998, t27999, t28001)
}
