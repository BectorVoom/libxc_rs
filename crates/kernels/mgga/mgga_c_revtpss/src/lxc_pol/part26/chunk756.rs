//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 756/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk756<F: Float>(t3989: F, t4014: F, t1370: F, t9697: F, t9700: F, t9705: F, t9711: F, t9712: F, t9716: F, t9725: F, t9729: F, t9735: F, t9739: F, t9742: F, t9745: F, t9748: F, t9750: F) -> F {
    let t9753 = t3989 * t4014;
    let t9755 = F::new(7.0) / F::new(48.0) * t9697 - t1370 * t9700 / F::new(48.0) - F::new(0.42874018118069736972e-3) * t9705 + t9711 - F::new(0.91464571985215438873e-3) * t9712 + F::new(0.85748036236139473944e-4) * t9716 + t9725 - t9729 - t9735 + F::new(0.30492001685571196935e-4) * t9739 - F::new(35.0) / F::new(72.0) * t9742 - F::new(7.0) / F::new(16.0) * t9745 - t9748 * t9750 / F::new(4.0) - F::new(0.60023625365297631762e-1) * t9753;
    t9755
}
