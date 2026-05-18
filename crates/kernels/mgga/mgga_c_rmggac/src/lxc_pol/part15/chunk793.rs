//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 793/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk793<F: Float>(t38704: F, t16156: F, t8808: F, t8504: F, t7345: F, t8349: F, t7335: F, t7508: F, t8533: F, t2134: F, t27: F, t3118: F, t551: F) -> (F, F, F, F, F, F, F) {
    let t38705 = F::new(0.17877131955185092547e-3) * t38704;
    let t38710 = t16156 * t8808;
    let t38712 = t16156 * t8504;
    let t38749 = t7345 * t8349;
    let t38757 = t7335 * t8349;
    let t38775 = t7508 * t8533;
    let t38776 = F::new(0.18183107769496894486e-1) * t38775;
    let t38784 = t2134 * t27 * t3118 * t551;
    (t38705, t38710, t38712, t38749, t38757, t38776, t38784)
}
