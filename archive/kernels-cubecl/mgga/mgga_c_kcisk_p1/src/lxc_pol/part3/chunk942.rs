//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 942/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk942<F: Float>(t13945: F, t3785: F, t1411: F, t1286: F, t3732: F, t1450: F, t1415: F, t10471: F, t1337: F, t140: F, t1343: F, t3480: F, t3737: F) -> (F, F, F, F, F) {
    let t13946 = t3785 * t13945;
    let t13947 = t1411 * t13946;
    let t13949 = t3732 * t1286;
    let t13950 = t1450 * t13949;
    let t13951 = t1415 * t13950;
    let t13952 = t1411 * t13951;
    let t13955 = t140 * t10471 * t1337;
    let t13956 = t13955 * t1343;
    let t13959 = t140 * t3737 * t3480;
    (t13947, t13949, t13952, t13956, t13959)
}
