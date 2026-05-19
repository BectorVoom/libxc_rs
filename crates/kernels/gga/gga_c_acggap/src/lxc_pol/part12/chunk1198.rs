//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1198/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1198<F: Float>(t35529: F, t31285: F, t31287: F, t31293: F, t31297: F, t31299: F, t31305: F, t31312: F, t31316: F, t31318: F, t31322: F, t31342: F, t31344: F, t32760: F, t32763: F, t32765: F, t32782: F, t35535: F) -> F {
    let t37591 = F::cast_from(0.68598428988911579156e-2_f64) * t35529;
    let t37603 = -t32760 - F::cast_from(0.21437009059034868486e-2_f64) * t31285 - F::cast_from(0.13719685797782315831e-1_f64) * t31287 + t32763 - t37591 + t31293 / F::new(16.0) - t32765 - F::cast_from(0.62896184579208304137e-2_f64) * t31297 + F::cast_from(0.56606566121287473724e-1_f64) * t31299 - F::cast_from(0.17149607247227894789e-2_f64) * t35535 - F::cast_from(0.80031500487063509014e-2_f64) * t31305 - F::cast_from(0.17149607247227894789e-2_f64) * t31312 + F::cast_from(0.25158473831683321654e-2_f64) * t31316 + F::cast_from(0.22642626448514989489e-1_f64) * t31318 + F::cast_from(0.17149607247227894789e-2_f64) * t31322 + t32782 + F::new(7.0) / F::new(72.0) * t31342 + F::new(7.0) / F::new(144.0) * t31344;
    t37603
}
