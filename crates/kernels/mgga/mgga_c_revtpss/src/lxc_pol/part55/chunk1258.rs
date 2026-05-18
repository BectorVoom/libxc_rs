//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1258/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1258<F: Float>(t121131: F, t128790: F, t121249: F, t122443: F, t122493: F, t122494: F, t122496: F, t125868: F, t27853: F, t27858: F, t32690: F, t32726: F, t34204: F, t7308: F, t7921: F, t7930: F) -> F {
    let t128812 = t121131 * t128790;
    let t128826 = F::new(0.37645955677973955999e-4) * t121249 + F::new(0.42839803248826764462e-1) * t128812 + F::new(0.17347256376410398924e1) * t122443 * t7921 - t122493 + t122494 - F::new(0.8673628188205199462e0) * t34204 * t7308 - F::new(0.8673628188205199462e0) * t32726 * t7930 + F::new(0.25389723392137995738e-1) * t122496 + F::new(0.7437465841810202164e-3) * t125868 + F::new(0.8673628188205199462e0) * t32690 * t27853 + F::new(0.8673628188205199462e0) * t32690 * t27858;
    t128826
}
