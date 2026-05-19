//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 954/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk954<F: Float>(t5: F, t2247: F, t32148: F, t6972: F, t8441: F, t8621: F, t32135: F, t640: F, t84: F, t32132: F, t32136: F, t32138: F, t32142: F, t32145: F, t8443: F) -> (F, F, F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t32149 = t2247 * t32148;
    let t32151 = t8621 * t8441 * t6972;
    let t32154 = t2247 * t32135;
    let t32156 = t8621 * t84 * t640;
    let t32160 = piecewise3::<F>(t8, F::new(0.0), F::new(5.0) / F::new(144.0) * t32132 * t8443 - F::new(5.0) / F::new(24.0) * t32136 * t32138 - F::new(5.0) / F::new(36.0) * t32142 * t32145 + F::new(5.0) / F::new(72.0) * t32149 * t32151 + F::new(5.0) / F::new(72.0) * t32154 * t32156);
    (t32149, t32151, t32154, t32156, t32160)
}
