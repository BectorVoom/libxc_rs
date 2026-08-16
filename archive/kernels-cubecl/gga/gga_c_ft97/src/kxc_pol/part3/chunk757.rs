//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 757/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk757<F: Float>(t15792: F, t1594: F, t15630: F, t4445: F, t7839: F, t35: F, t938: F, t401: F, t1711: F, t25: F, t371: F, t428: F) -> (F, F, F, F, F, F) {
    let t15793 = t1594 * t15792;
    let t15797 = t1594 * t15630;
    let t15802 = t4445 * t7839;
    let t15805 = t35 * t938;
    let t15806 = t15805 * t401;
    let t15810 = t1711 * t25;
    let t15811 = t371 * t15810;
    let t15812 = t15805 * t428;
    (t15793, t15797, t15802, t15806, t15811, t15812)
}
