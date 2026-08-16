//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2340/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2340<F: Float>(t14165: F, t42841: F, t10254: F, t12652: F, t10241: F, t10259: F, t13835: F, t13839: F, t13861: F, t17748: F, t2986: F, t2988: F, t42889: F, t42893: F, t42895: F, t42903: F, t42906: F, t43065: F, t4518: F, t47701: F) -> (F, F) {
    let t47941 = t42841 * t14165;
    let t47966 = t10254 * t12652;
    let t47978 = F::cast_from(0.27160493827160493826e-2_f64) * t42889 + F::cast_from(0.3086419753086419753e-3_f64) * t42893 - F::cast_from(0.54320987654320987653e-2_f64) * t42895 + F::cast_from(0.16666666666666666666e-2_f64) * t2986 * t10241 * t13835 - F::cast_from(0.11111111111111111111e-2_f64) * t2986 * t43065 * t13839 - F::cast_from(0.83333333333333333331e-3_f64) * t2986 * t10241 * t17748 + F::cast_from(0.33333333333333333333e-2_f64) * t2986 * t2988 * t47966 - F::cast_from(0.83333333333333333331e-3_f64) * t2986 * t10259 * t13861 + F::cast_from(0.49999999999999999999e-2_f64) * t2986 * t4518 * t47701 - F::cast_from(0.9259259259259259259e-3_f64) * t42903 + F::cast_from(0.55555555555555555554e-3_f64) * t42906;
    (t47941, t47978)
}
