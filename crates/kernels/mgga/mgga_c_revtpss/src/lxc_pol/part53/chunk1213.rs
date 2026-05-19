//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1213/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1213<F: Float>(t5: F, t129196: F, t129243: F, t117: F, t32855: F, t4248: F, t27123: F, t8749: F, t27126: F, t32866: F, t7732: F, t1310: F, t25805: F, t28025: F, t28030: F, t29444: F, t29459: F, t32825: F, t34419: F, t4297: F, t508: F, t6985: F, t7591: F, t8158: F) -> (F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t129245 = piecewise3::<F>(t8, F::new(0.0), t129196 + t129243);
    let t129246 = t129245 * t117;
    let t129251 = t4248 * t32855;
    let t129253 = t27123 * t8749;
    let t129255 = t27126 * t8749;
    let t129257 = t7732 * t32866;
    let t129265 = -t129246 * t508 - t1310 * t34419 - F::new(2.0) * t25805 * t8158 - F::new(2.0) * t28025 * t8158 - F::new(2.0) * t28030 * t7591 - F::new(2.0) * t29444 * t6985 - F::new(2.0) * t29459 * t6985 - F::new(2.0) * t32825 * t4297 - F::new(2.0) * t129251 - F::new(2.0) * t129253 - F::new(2.0) * t129255 - F::new(2.0) * t129257;
    (t129246, t129265)
}
