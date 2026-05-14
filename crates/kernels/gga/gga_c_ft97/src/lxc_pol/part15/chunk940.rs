//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 940/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk940<F: Float>(t86981: F, t87016: F, t87050: F, t87086: F, t17409: F, t4805: F, t20897: F, t50260: F, t12664: F, t20902: F, t4724: F, t61366: F, t39674: F, t61462: F, t62134: F, t86608: F, t86937: F, t86942: F, t86946: F, t86950: F, t86954: F, t86958: F, t86962: F, t86966: F, t86970: F, t86975: F, t86979: F) -> (F, F, F, F, F, F) {
    let t87088 = t86981 + t87016 + t87050 + t87086;
    let t87091 = t17409 * t4805;
    let t87093 = t50260 * t20897;
    let t87095 = t12664 * t20902;
    let t87097 = t61366 * t4724;
    let t87113 = -15.0 / 16.0 * t86608 + t86937 / 2.0 + 16.0 / 9.0 * t61462 + t39674 - 36.0 * t86942 + 8.0 * t86946 - 80.0 / 81.0 * t86950 - 8.0 * t86954 - t86958 / 3.0 + 8.0 * t86962 + 2.0 * t86966 - 2.0 / 3.0 * t86970 + 16.0 / 3.0 * t62134 - 8.0 * t86975 + 8.0 / 3.0 * t86979;
    (t87088, t87091, t87093, t87095, t87097, t87113)
}
