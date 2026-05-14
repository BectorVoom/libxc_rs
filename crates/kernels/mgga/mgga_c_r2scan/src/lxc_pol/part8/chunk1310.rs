//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1310/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1310<F: Float>(t898: F, t9005: F, t10297: F, t2271: F, t10265: F, t424: F, t2854: F, t2858: F, t9115: F, t18908: F, t23711: F, t23715: F, t31365: F, t32093: F, t32097: F, t32106: F, t32108: F, t881: F) -> (F, F, F, F) {
    let t32111 = t898 * t9005;
    let t32114 = t2271 * t10297;
    let t32116 = t424 * t10265;
    let t32121 = 18.0 * t2858 * t2854 * t9115;
    let t32122 = -t32093 + 3.0 * t31365 - 0.7089e1 * t32106 + t32097 - t23711 - t18908 - 0.7089e1 * t881 * t32108 - 0.7089e1 * t881 * t32111 - 0.2363e1 * t32114 - 0.2363e1 * t881 * t32116 - t32121 - t23715;
    (t32111, t32116, t32121, t32122)
}
