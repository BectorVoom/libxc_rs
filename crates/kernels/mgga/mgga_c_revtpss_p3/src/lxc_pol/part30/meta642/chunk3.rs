//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2240/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2240<F: Float>(t17617: F, t26870: F, t3682: F, t8172: F, t29020: F, t3704: F, t29086: F, t3678: F, t16733: F, t16738: F, t16742: F, t17515: F, t29047: F, t29048: F, t97174: F, t97267: F, t97269: F, t97272: F) -> F {
    let t104953 = F::cast_from(0.57165357490759649296e-3_f64) * t26870 * t17617;
    let t104963 = t8172 * t3682;
    let t104968 = F::cast_from(0.30488190661738479624e-2_f64) * t29020 * t3704;
    let t104972 = F::cast_from(0.57165357490759649296e-3_f64) * t29086 * t3678;
    let t104973 = -t104953 - t29047 * t29048 * t16738 / F::new(72.0) - t29047 * t29048 * t16742 / F::new(144.0) - t29047 * t29048 * t16733 / F::new(48.0) + t104963 / F::new(162.0) - F::cast_from(0.19055119163586549765e-3_f64) * t97267 + F::cast_from(0.28582678745379824648e-3_f64) * t97269 + t97272 - t104968 + F::cast_from(0.57165357490759649296e-3_f64) * t97174 * t17515 - t104972;
    t104973
}
