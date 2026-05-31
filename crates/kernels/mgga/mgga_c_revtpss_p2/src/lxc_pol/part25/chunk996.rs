//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 996/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk996<F: Float>(t11165: F, t4915: F, t1066: F, t11169: F, t247: F, t1011: F, t1025: F, t1063: F, t11802: F, t11806: F, t11811: F, t11814: F, t11818: F, t11824: F, t11829: F, t11836: F, t3177: F, t3184: F, t3188: F, t3241: F, t3248: F, t3255: F, t4837: F) -> (F, F) {
    let t11839 = t4915 * t11165;
    let t11845 = t247 * t1066 * t11169;
    let t11850 = F::cast_from(0.57165357490759649295e-3_f64) * t11802 + F::cast_from(0.12862205435420921092e-2_f64) * t4837 * t11806 - F::cast_from(0.21437009059034868486e-3_f64) * t1025 * t11811 + F::cast_from(0.45732285992607719436e-2_f64) * t11814 + F::cast_from(0.14291339372689912324e-3_f64) * t11818 + F::cast_from(7.0_f64) / F::cast_from(648.0_f64) * t1011 * t11824 - t1011 * t11829 / F::cast_from(36.0_f64) - t3241 * t3248 / F::cast_from(36.0_f64) - t3241 * t3255 / F::cast_from(27.0_f64) + t1011 * t11836 / F::cast_from(72.0_f64) - t1011 * t11839 / F::cast_from(48.0_f64) + F::cast_from(0.42874018118069736972e-3_f64) * t3188 * t3177 + F::cast_from(0.14291339372689912324e-3_f64) * t1063 * t11845 + F::cast_from(0.7145669686344956162e-3_f64) * t3188 * t3184;
    (t11845, t11850)
}
