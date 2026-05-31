//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 791/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk791<F: Float>(t33953: F, t852: F, t1486: F, t193: F, t375: F, t7654: F, t89: F, t668: F, t7584: F) -> (F, F, F, F, F) {
    let t33954 = t852 * t33953;
    let t33956 = t1486 * t193 * t33954;
    let t33959 = t89 * t375 * t7654;
    let t33960 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t33959;
    let t33961 = t7584 * t668;
    (t33954, t33956, t33959, t33960, t33961)
}
