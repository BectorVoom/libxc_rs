//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1023/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1023<F: Float>(t32238: F, t32240: F, t32188: F, t32191: F, t32198: F, t32203: F, t32209: F, t32214: F, t32222: F, t32225: F, t32226: F, t32230: F, t32233: F, t32234: F, t7308: F, t8579: F) -> (F, F) {
    let t32242 = F::new(0.14279934416275588154e-1) * t32238 * t32240;
    let t32243 = -t32188 + t32191 - F::new(0.28234466758480466999e-3) * t32198 - t32203 - F::new(0.112937867033921868e-2) * t32209 - F::new(0.28234466758480466999e-3) * t32214 + t32222 - t32225 - F::new(0.17347256376410398924e1) * t32226 * t7308 + F::new(0.17347256376410398924e1) * t8579 * t32230 + F::new(0.8673628188205199462e0) * t32233 * t32234 - t32242;
    (t32242, t32243)
}
