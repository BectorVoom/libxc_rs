//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 783/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk783<F: Float>(t13863: F, t3892: F, t2606: F, t265: F, t668: F, t724: F, t1144: F, t8232: F, t1882: F, t3991: F, t3887: F, t9787: F, t2486: F, t754: F, t3893: F, t3899: F, t8392: F) -> (F, F, F, F, F, F, F) {
    let t13864 = t3892 * t13863;
    let t13865 = t2606 * t13864;
    let t13869 = t724 * t265 * t668;
    let t13872 = t8232 * t1144;
    let t13875 = 2.0 / 9.0 * t1882 * t3991;
    let t13876 = t9787 * t3887;
    let t13879 = t2486 * t754;
    let t13880 = t13879 * t3893;
    let t13884 = 2.0 / 27.0 * t8392 * t3899;
    (t13865, t13869, t13872, t13875, t13876, t13880, t13884)
}
