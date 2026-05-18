//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1203/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1203<F: Float>(t35709: F, t31501: F, t31503: F, t31505: F, t31510: F, t31514: F, t31526: F, t31528: F, t31530: F, t31532: F, t32833: F, t32834: F, t32839: F, t32844: F, t32850: F, t35720: F, t35722: F, t35724: F) -> F {
    let t37675 = F::new(0.64025200389650807212e-1) * t35709;
    let t37688 = -t32833 - t32834 - t37675 + F::new(0.64311027177104605458e-2) * t31501 - F::new(0.77173232612525526552e-2) * t31503 - F::new(0.36014175219178579058e-1) * t31505 - t32839 - F::new(7.0) / F::new(72.0) * t31510 - F::new(11.0) / F::new(288.0) * t31514 + t32844 + F::new(0.79249192569802463213e-1) * t31526 + F::new(0.22642626448514989489e-1) * t31528 + F::new(0.68598428988911579156e-2) * t31530 - F::new(0.68598428988911579156e-2) * t31532 + F::new(0.34299214494455789578e-1) * t35720 + t32850 + F::new(0.51448821741683684366e-2) * t35722 + F::new(0.13719685797782315831e-1) * t35724;
    t37688
}
