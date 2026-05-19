//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 954/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk954<F: Float>(t5: F, t10309: F, t33358: F, t38: F, t8911: F, t2247: F, t7574: F, t8441: F, t8621: F, t32132: F, t32138: F, t32145: F, t32156: F, t8737: F, t8913: F) -> (F, F, F, F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t33359 = t10309 * t33358;
    let t33362 = t38 * t8911;
    let t33363 = t2247 * t33362;
    let t33367 = t8621 * t8441 * t7574;
    let t33370 = t2247 * t33358;
    let t33374 = piecewise3::<F>(t8, F::new(0.0), F::new(5.0) / F::new(144.0) * t32132 * t8913 - F::new(5.0) / F::new(24.0) * t33359 * t32138 - F::new(5.0) / F::new(36.0) * t33363 * t32145 + F::new(5.0) / F::new(72.0) * t8737 * t33367 + F::new(5.0) / F::new(72.0) * t33370 * t32156);
    (t33359, t33362, t33363, t33367, t33370, t33374)
}
