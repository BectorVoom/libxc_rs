//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 777/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk777<F: Float>(t1614: F, t1971: F, t495: F, t511: F, t7230: F, t2333: F, t34957: F, t356: F, t638: F, t639: F, t8849: F, t34750: F, t34755: F, t577: F, t2392: F, t866: F) -> (F, F, F, F, F) {
    let t39360 = t7230 * t1971 * t511 * t1614 * t495;
    let t39362 = t34957 * t2333;
    let t39367 = t638 * t639 * t8849 * t356;
    let t39370 = t34755 * t577 * t34750;
    let t39372 = t2392 * t866;
    (t39360, t39362, t39367, t39370, t39372)
}
