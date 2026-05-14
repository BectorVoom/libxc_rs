//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 888/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk888<F: Float>(t2290: F, t7630: F, t1549: F, t30540: F, t1554: F, t1558: F, t30137: F, t7585: F, t8525: F, t1072: F, t535: F, t7507: F, t7512: F, t7447: F, t8924: F, t7440: F, t8929: F) -> (F, F, F, F, F, F, F, F) {
    let t34849 = t7630 * t2290;
    let t34851 = t30540 * t1549;
    let t34853 = t30540 * t1554;
    let t34855 = t30540 * t1558;
    let t34856 = 0.40015750243531754508e-2 * t34855;
    let t34865 = t7585 * t30137 * t8525;
    let t34866 = 0.14291339372689912324e-3 * t34865;
    let t34879 = t7507 * t7512 * t535 * t1072;
    let t34893 = t7447 * t8924;
    let t34894 = 0.16809375e0 * t34893;
    let t34895 = t7440 * t8929;
    (t34849, t34851, t34853, t34856, t34866, t34879, t34894, t34895)
}
