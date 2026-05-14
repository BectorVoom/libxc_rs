//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 541/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk541<F: Float>(t4883: F, t637: F, t639: F, t2251: F, t2265: F, t3611: F, t3633: F, t4857: F, t4861: F, t4865: F, t4869: F, t4874: F, t631: F, t184: F, t21: F, t1078: F) -> (F, F, F, F, F) {
    let t4885 = t637 * t639 * t4883;
    let t4888 = -t2251 - 2.0 / 9.0 * t3611 - 2.0 / 3.0 * t3633 + t631 * t4857 / 18.0 - 2.0 / 3.0 * t2265 * t4861 - t631 * t4865 / 3.0 + t631 * t4869 / 6.0 - 3.0 / 2.0 * t631 * t4874 + t631 * t4885 / 2.0;
    let t4889 = t4888 * t184;
    let t4890 = t4889 * t21;
    let t4893 = t1078 * t1078;
    (t4885, t4888, t4889, t4890, t4893)
}
