//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 710/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk710<F: Float>(t7592: F, t7523: F, t2414: F, t777: F, t216: F, t231: F, t2417: F, t228: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7593 = F::new(0.36793333333333333333e0) * t7592;
    let t7594 = F::new(0.93932222222222222223e0) * t7523;
    let t7609 = F::new(0.28842592592592592592e-1) * t7523;
    let t7656 = F::new(0.36514074074074074075e0) * t7592;
    let t7657 = F::new(0.93011851851851851854e0) * t7523;
    let t7668 = F::new(1.0) / t2414 / t777;
    let t7669 = t216 * t7668;
    let t7672 = F::new(1.0) / t2417 / t231;
    let t7680 = F::new(1.0) / t2414 / t228;
    let t7681 = t216 * t7680;
    let t7699 = F::new(0.53272592592592592592e-1) * t7523;
    let t7713 = F::new(0.55403703703703703703e-1) * t7523;
    (t7593, t7594, t7609, t7656, t7657, t7668, t7669, t7672, t7680, t7681, t7699, t7713)
}
