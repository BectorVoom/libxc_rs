//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 897/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk897<F: Float>(t18: F, t713: F, t2600: F, t2599: F, t766: F, t2607: F, t2606: F, t1882: F, t3999: F, t3995: F, t1175: F, t2373: F, t2574: F) -> (F, F, F, F, F) {
    let t13892 = t18 * t713;
    let t13893 = t2600 * t13892;
    let t13894 = t2599 * t13893;
    let t13897 = t18 * t766;
    let t13898 = t2607 * t13897;
    let t13899 = t2606 * t13898;
    let t13903 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1882 * t3999;
    let t13905 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1882 * t3995;
    let t13907 = t2574 * t1175 * t2373;
    (t13894, t13899, t13903, t13905, t13907)
}
