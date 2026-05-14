//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 805/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk805<F: Float>(t3889: F, t852: F, t3919: F, t3937: F, t3868: F, t1264: F, t449: F, t863: F, t864: F, t3101: F, t322: F, t317: F, t3054: F, t441: F, t865: F, t3912: F, t868: F) -> (F, F, F, F, F, F, F, F) {
    let t12241 = t852 * t3889;
    let t12243 = t3937 * t3919;
    let t12246 = t3868 * t3919;
    let t12250 = t863 * t449 * t864 * t1264;
    let t12254 = t322 * t3101;
    let t12257 = 0.52683593463484092788e1 * t863 * t317 * t12254;
    let t12259 = t3054 * t441 * t865;
    let t12263 = t868 * t3912;
    (t12241, t12243, t12246, t12250, t12254, t12257, t12259, t12263)
}
