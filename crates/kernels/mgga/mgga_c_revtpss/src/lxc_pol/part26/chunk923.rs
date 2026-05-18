//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 923/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk923<F: Float>(t11165: F, t4915: F, t1066: F, t11169: F, t247: F, t1011: F, t1025: F, t1063: F, t11802: F, t11806: F, t11811: F, t11814: F, t11818: F, t11824: F, t11829: F, t11836: F, t3177: F, t3184: F, t3188: F, t3241: F, t3248: F, t3255: F, t4837: F) -> F {
    let t11839 = t4915 * t11165;
    let t11845 = t247 * t1066 * t11169;
    let t11850 = F::new(0.57165357490759649295e-3) * t11802 + F::new(0.12862205435420921092e-2) * t4837 * t11806 - F::new(0.21437009059034868486e-3) * t1025 * t11811 + F::new(0.45732285992607719436e-2) * t11814 + F::new(0.14291339372689912324e-3) * t11818 + F::new(7.0) / F::new(648.0) * t1011 * t11824 - t1011 * t11829 / F::new(36.0) - t3241 * t3248 / F::new(36.0) - t3241 * t3255 / F::new(27.0) + t1011 * t11836 / F::new(72.0) - t1011 * t11839 / F::new(48.0) + F::new(0.42874018118069736972e-3) * t3188 * t3177 + F::new(0.14291339372689912324e-3) * t1063 * t11845 + F::new(0.7145669686344956162e-3) * t3188 * t3184;
    t11850
}
