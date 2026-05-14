//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 961/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk961<F: Float>(t1549: F, t30540: F, t1554: F, t1558: F, t4695: F, t7822: F, t4335: F, t2068: F, t4680: F, t8521: F, t30137: F, t7585: F, t8525: F, t2030: F, t301: F, t4262: F, t8484: F) -> (F, F, F, F, F, F, F, F) {
    let t34851 = t30540 * t1549;
    let t34853 = t30540 * t1554;
    let t34855 = t30540 * t1558;
    let t34856 = 0.40015750243531754508e-2 * t34855;
    let t34857 = t7822 * t4695;
    let t34859 = t7822 * t4335;
    let t34862 = t2068 * t4680 * t8521;
    let t34865 = t7585 * t30137 * t8525;
    let t34866 = 0.14291339372689912324e-3 * t34865;
    let t34869 = t2030 * t4262 * t8484 * t301;
    (t34851, t34853, t34856, t34857, t34859, t34862, t34866, t34869)
}
