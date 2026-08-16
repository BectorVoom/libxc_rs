//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1008/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1008<F: Float>(t22287: F, t6348: F, t6500: F, t1772: F, t1796: F, t1990: F, t1867: F, t22120: F, t601: F, t6424: F, t768: F, t97: F) -> (F, F, F, F) {
    let t22293 = F::cast_from(0.1926377843805564792e1_f64) * t22287 * t6500 * t6348;
    let t22296 = F::cast_from(0.86748647062252193713e-1_f64) * t1796 * t1772 * t1990;
    let t22300 = F::cast_from(0.6233672123775310788e3_f64) * t601 * t6424 * t22120 * t1867;
    let t22308 = F::cast_from(1.0_f64) / t97 / t768;
    (t22293, t22296, t22300, t22308)
}
