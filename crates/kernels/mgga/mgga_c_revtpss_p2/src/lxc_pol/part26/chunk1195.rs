//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1195/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1195<F: Float>(t30: F, t265: F, t393: F, t96072: F, t10326: F, t2078: F, t2258: F, t26626: F, t45: F, t606: F, t7449: F, t95972: F, t96016: F, t1940: F, t2071: F, t2082: F, t2403: F, t25767: F, t25784: F, t26425: F, t26585: F, t26590: F, t28291: F, t28472: F, t33: F, t7428: F, t7432: F, t92822: F, t94228: F, t94231: F, t94234: F, t94240: F, t94246: F, t94259: F, t94276: F, t94280: F, t94293: F, t94297: F, t94316: F, t95954: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t96073 = piecewise3::<F>(t394, F::new(0.0), t96072);
    let t96083 = piecewise3::<F>(t120, t95972 + t96016, t96073 * t45 / F::new(2.0) + F::new(3.0) / F::new(2.0) * t26626 * t606 + F::new(3.0) / F::new(2.0) * t7449 * t2258 + t2078 * t10326 / F::new(2.0));
    let t96121 = F::new(3.0) * t1940 * t26590 * t94316 + F::new(3.0) * t92822 * t2082 - F::new(3.0) / F::new(2.0) * t1940 * t7432 * t94276 + F::new(9.0) / F::new(2.0) * t2403 * t7428 * t25767 + F::new(9.0) / F::new(2.0) * t2403 * t2071 * t94293 - F::new(3.0) / F::new(2.0) * t1940 * t26585 * t25784 - F::new(9.0) / F::new(2.0) * t26425 * t94228 + t1940 * t95954 * t33 / F::new(2.0) + F::new(9.0) / F::new(2.0) * t2403 * t2071 * t94297 + F::new(3.0) * t28472 * t94234 + F::new(9.0) * t26425 * t94231 - F::new(9.0) * t28291 * t94240 - F::new(9.0) * t26425 * t94246 + F::new(9.0) * t28291 * t94280 - F::new(9.0) / F::new(2.0) * t26425 * t94259;
    (t96083, t96121)
}
