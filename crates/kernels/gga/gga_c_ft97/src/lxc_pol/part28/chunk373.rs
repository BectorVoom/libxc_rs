//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 373/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk373<F: Float>(t1852: F, t5731: F, t83: F, t5672: F, t5689: F, t5669: F, t5678: F, t5682: F, t5686: F, t5694: F, t5698: F, t5702: F) -> (F, F, F, F) {
    let t5732 = t1852 * t5731;
    let t5733 = t83 * t5732;
    let t5737 = t5672 / F::cast_from(6.0_f64);
    let t5740 = t5689 / F::cast_from(3.0_f64);
    let t5743 = t5669 / F::cast_from(4.0_f64) + t5737 + t5678 / F::cast_from(6.0_f64) + t5682 - t5686 / F::cast_from(2.0_f64) + t5740 + t5694 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) * t5698 - t5702;
    (t5733, t5737, t5740, t5743)
}
