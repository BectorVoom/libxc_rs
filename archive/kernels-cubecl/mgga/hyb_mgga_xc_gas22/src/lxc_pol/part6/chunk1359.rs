//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1359/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1359<F: Float>(t11067: F, t2562: F, t8980: F, t9258: F, t8990: F, t9104: F, t11019: F, t7070: F, t11022: F, t6951: F, t11025: F, t21382: F) -> (F, F, F, F, F, F) {
    let t29602 = t11067 * t2562;
    let t29627 = F::cast_from(8.0_f64) * t9258 * t8980;
    let t29629 = F::cast_from(0.64327917994770140268e2_f64) * t9104 * t8990;
    let t29631 = F::cast_from(12.0_f64) * t7070 * t11019;
    let t29633 = F::cast_from(8.0_f64) * t6951 * t11022;
    let t29635 = F::cast_from(0.1929837539843104208e3_f64) * t21382 * t11025;
    (t29602, t29627, t29629, t29631, t29633, t29635)
}
