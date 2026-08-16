//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1219/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1219<F: Float>(t13763: F, t8546: F, t2494: F, t944: F, t1167: F, t2182: F, t3324: F, t2074: F, t1172: F, t1105: F, t2423: F, t2051: F) -> (F, F, F, F, F, F, F, F) {
    let t52775 = t8546 * t13763;
    let t52782 = t2494 * t944;
    let t52791 = t1167 * t2182;
    let t52829 = t3324 * t944;
    let t52837 = t1167 * t2074;
    let t52841 = t1172 * t2182;
    let t52847 = t1105 * t2423;
    let t52870 = t1167 * t2051;
    (t52775, t52782, t52791, t52829, t52837, t52841, t52847, t52870)
}
