//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1023/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1023<F: Float>(t12938: F, t629: F, t632: F, t12939: F, t1625: F, t209: F, t736: F, t4188: F, t5895: F, t12344: F, t2016: F, t2118: F, t1943: F, t38630: F, t17329: F, t1363: F, t16349: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F) {
    let t40653 = t629 / t12938 / t632;
    let t40662 = t1625 * t12939;
    let t44682 = t209 * t736;
    let t48044 = t5895 * t4188;
    let t48058 = t2016 * t12344;
    let t51097 = t2118 * t12939;
    let t51121 = t1943 * t38630;
    let t51125 = t17329 * sigma2;
    let t51602 = t16349 * t1363;
    (t40653, t40662, t44682, t48044, t48058, t51097, t51121, t51125, t51602)
}
