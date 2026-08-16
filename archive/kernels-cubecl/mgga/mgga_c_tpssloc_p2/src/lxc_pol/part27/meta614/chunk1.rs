//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2090/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2090<F: Float>(t1937: F, t607: F, t6722: F, t10375: F, t1942: F, t1036: F, t23551: F, t23562: F, t343: F, t83032: F, t210: F, t23322: F) -> (F, F, F, F, F) {
    let t83075 = t6722 * t607 * t1937;
    let t83080 = t1942 * t10375 / F::cast_from(5184.0_f64);
    let t83082 = t23551 * t1036;
    let t83085 = t23562 * t83032 * t343;
    let t83092 = t23322 * t210;
    (t83075, t83080, t83082, t83085, t83092)
}
