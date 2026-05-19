//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1114/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1114<F: Float>(t1784: F, t2020: F, t30664: F, t30670: F, t30672: F, t30673: F, t30715: F, t30717: F, t34660: F, t34675: F, t34703: F, t34704: F, t34711: F, t34713: F, t34718: F, t37211: F, t37220: F, t37221: F, t37225: F) -> F {
    let t39427 = t2020 * t1784;
    let t39432 = -t30664 - t30670 + t30672 - F::cast_from(0.17149607247227894789e-2_f64) * t30673 + t34660 + F::cast_from(0.41930789719472202757e-3_f64) * t34675 - F::new(7.0) / F::new(144.0) * t39427 - t37211 - t34703 - F::cast_from(0.77173232612525526552e-2_f64) * t34704 + t34711 + t34713 - F::cast_from(0.51448821741683684367e-2_f64) * t34718 + t37220 + t37221 + t37225 - t30715 - F::new(35.0) / F::new(216.0) * t30717;
    t39432
}
