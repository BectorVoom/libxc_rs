//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1355/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1355<F: Float>(t2014: F, t25089: F, t25190: F, t28167: F, t49616: F, t8717: F, t1450: F, t2033: F, t9400: F, t10192: F, t1310: F, t13207: F, t1932: F, t2011: F, t2328: F, t2371: F, t2372: F, t25096: F, t25800: F, t28025: F, t3813: F, t4151: F, t508: F, t651: F, t670: F, t6983: F, t7221: F, t7231: F, t94947: F, t95073: F, t95075: F, t95081: F, t95083: F, t95085: F, t95087: F, t95090: F) -> F {
    let t95096 = F::cast_from(9.0_f64) * t2014 * t25190 * t25089;
    let t95104 = F::cast_from(18.0_f64) * t28167 * t8717 * t49616;
    let t95108 = F::cast_from(6.0_f64) * t2014 * t9400 * t2033 * t1450;
    let t95117 = -F::cast_from(6.0_f64) * t2371 * t651 * t7221 - F::cast_from(6.0_f64) * t25800 * t651 * t670 + t10192 * t2011 - F::cast_from(6.0_f64) * t1310 * t25096 - t13207 * t1932 - F::cast_from(6.0_f64) * t2328 * t7221 - F::cast_from(6.0_f64) * t2372 * t28025 - F::cast_from(3.0_f64) * t3813 * t6983 + F::cast_from(3.0_f64) * t4151 * t7231 - F::cast_from(6.0_f64) * t508 * t94947 - t95073 - t95075 + t95081 - t95083 - t95085 - t95087 - t95090 + t95096 - t95104 + t95108;
    t95117
}
