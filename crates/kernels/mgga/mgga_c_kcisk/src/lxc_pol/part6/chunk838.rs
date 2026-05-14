//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 838/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk838<F: Float>(t28699: F, t716: F, t740: F, t748: F, t2567: F, t9089: F, t5284: F, t2586: F, t9050: F, t5315: F, t29274: F, t4971: F, t735: F, t734: F, t28303: F, t7311: F) -> (F, F, F, F, F) {
    let t29584 = t28699 * t716;
    let t29585 = t29584 * t740;
    let t29586 = t29585 * t748;
    let t29588 = t2567 * t9089;
    let t29589 = t5284 * t29588;
    let t29590 = t2586 * t9050;
    let t29591 = t5315 * t29590;
    let t29593 = t4971 * t29274;
    let t29594 = t735 * t29593;
    let t29595 = t734 * t29594;
    let t29597 = t7311 * t28303;
    (t29586, t29589, t29591, t29595, t29597)
}
