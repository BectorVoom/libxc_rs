//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1272/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1272<F: Float>(t1882: F, t31255: F, t31207: F, t31231: F, t31152: F, t1091: F, t110369: F, t110962: F, t110988: F, t1160: F, t13830: F, t18717: F, t1901: F, t2574: F, t28098: F, t28298: F, t28301: F, t28404: F, t3821: F, t3898: F, t446: F, t4934: F, t4973: F, t6187: F, t6194: F, t6861: F, t6947: F, t724: F, t729: F, t762: F, t97889: F) -> (F,) {
    let t124436 = t1882 * t31255;
    let t124447 = t1882 * t31207;
    let t124453 = t1882 * t31231;
    let t124455 = t1882 * t31152;
    let t124476 = 8.0 / 81.0 * t110962 - 2.0 / 9.0 * t124436 + 4.0 / 27.0 * t97889 - 2.0 / 27.0 * t1901 * t28404 * t18717 - 2.0 / 3.0 * t446 * t2574 * t762 * t6187 * t4934 + 2.0 / 81.0 * t124447 - t446 * t724 * t6194 * t4973 / 9.0 + 2.0 / 3.0 * t124453 - 2.0 / 9.0 * t124455 - 2.0 / 3.0 * t446 * t729 * t6947 * t3821 + t110988 + 2.0 / 9.0 * t1901 * t110369 * t3898 - 4.0 * t1901 * t28298 * t1160 * t28301 - 2.0 / 9.0 * t446 * t724 * t28098 * t1091 + 2.0 / 3.0 * t446 * t729 * t13830 * t6861;
    (t124476,)
}
