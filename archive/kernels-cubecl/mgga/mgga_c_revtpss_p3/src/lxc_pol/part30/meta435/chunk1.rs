//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1666/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1666<F: Float>(t141: F, t16907: F, t16708: F, t16710: F, t16712: F, t12296: F, t12297: F, t12299: F, t12301: F, t12303: F, t16706: F, t16717: F, t16722: F, t16727: F, t16731: F, t16735: F, t16740: F, t16744: F, t16748: F) -> (F, F) {
    let t16908 = t141 * t16907;
    let t16915 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t16708;
    let t16916 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t16710;
    let t16917 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t16712;
    let t16926 = -t12296 + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t12297 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t12299 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t12301 - t12303 / F::cast_from(9.0_f64) + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t16706 + t16915 - t16916 - t16917 + F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t16717 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t16722 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t16727 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t16731 + F::cast_from(2.0_f64) * t16735 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t16740 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t16744 + t16748 / F::cast_from(3.0_f64);
    (t16908, t16926)
}
