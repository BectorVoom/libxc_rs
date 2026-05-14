//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 597/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk597<F: Float>(t15781: F, t3020: F, t15630: F, t7906: F, t1597: F, t929: F, t1594: F, t35: F, t938: F, t1711: F, t25: F, t371: F, t173: F, t4479: F, t419: F, t4483: F) -> (F, F, F, F, F, F, F, F) {
    let t15782 = t3020 * t15781;
    let t15789 = t7906 * t15630;
    let t15792 = t929 * t1597;
    let t15793 = t1594 * t15792;
    let t15797 = t1594 * t15630;
    let t15805 = t35 * t938;
    let t15810 = t1711 * t25;
    let t15811 = t371 * t15810;
    let t15839 = t173 * t4479;
    let t15840 = t419 * t15839;
    let t15854 = t173 * t4483;
    (t15782, t15789, t15793, t15797, t15805, t15811, t15840, t15854)
}
