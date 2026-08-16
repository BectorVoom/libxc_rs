//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1299/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1299<F: Float>(t14732: F, t2484: F, t2652: F, t4435: F, t4343: F, t854: F, t236: F, t807: F, t221: F, t4433: F, t10703: F, t2674: F) -> (F, F, F, F, F, F, F) {
    let t14734 = F::cast_from(0.25410001404642664112e-4_f64) * t2484 * t14732;
    let t14736 = F::cast_from(0.40015750243531754508e-1_f64) * t2652 * t4435;
    let t14741 = t854 * t4343;
    let t14742 = t236 * t14741;
    let t14744 = F::cast_from(0.57165357490759649296e-4_f64) * t807 * t14742;
    let t14756 = t221 * t4433;
    let t14757 = t10703 * t14756;
    let t14759 = F::cast_from(0.50820002809285328225e-3_f64) * t2674 * t14757;
    (t14734, t14736, t14741, t14744, t14756, t14757, t14759)
}
