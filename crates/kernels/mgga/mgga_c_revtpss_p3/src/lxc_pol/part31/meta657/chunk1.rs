//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2215/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2215<F: Float>(t25978: F, t6856: F, t102569: F, t108615: F, t108617: F, t108619: F, t108623: F, t108625: F, t108627: F, t94554: F, t94565: F, t94569: F, t94571: F, t98282: F) -> F {
    let t108629 = t25978 * t6856;
    let t108631 = -F::cast_from(0.15244095330869239812e-3_f64) * t94554 + t108615 / F::cast_from(16.0_f64) - t108617 / F::cast_from(4.0_f64) + t108619 / F::cast_from(8.0_f64) + t98282 - F::cast_from(0.90357964994909313586e-5_f64) * t94565 - t94569 - t94571 - t102569 + F::cast_from(0.14291339372689912324e-4_f64) * t108623 + F::cast_from(0.50820002809285328226e-3_f64) * t108625 - F::cast_from(0.40015750243531754508e-1_f64) * t108627 + F::cast_from(0.80031500487063509015e-2_f64) * t108629;
    t108631
}
