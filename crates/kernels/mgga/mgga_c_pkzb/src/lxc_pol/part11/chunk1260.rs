//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1260/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1260<F: Float>(t1143: F, t3694: F, t10942: F, t306: F, t1123: F, t3669: F, t1133: F, t3638: F, t3650: F, t2036: F, t10979: F, t5955: F) -> (F, F, F, F, F, F, F, F) {
    let t30843 = t1143 * t3694;
    let t30868 = t306 * t10942;
    let t30885 = t3669 * t1123;
    let t30893 = t1133 * t3638;
    let t30897 = t1133 * t3650;
    let t30898 = t2036 * t30897;
    let t30910 = t306 * t10979;
    let t30916 = t5955 * t3650;
    (t30843, t30868, t30885, t30893, t30897, t30898, t30910, t30916)
}
