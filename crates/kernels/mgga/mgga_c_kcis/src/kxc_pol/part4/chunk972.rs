//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 972/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk972<F: Float>(t1071: F, t240: F, t9: F, t2866: F, t990: F, t2872: F, t2881: F, t2880: F, t2900: F, t991: F, t109: F, t992: F) -> (F, F, F, F, F) {
    let t9896 = F::new(1.0) / t240 / t1071;
    let t9897 = t9 * t9896;
    let t9903 = t2866 * t990;
    let t9906 = t2872 * t2881;
    let t9909 = t2880 * t2900;
    let t9910 = t991 * t9909;
    let t9916 = t109 * t992;
    (t9897, t9903, t9906, t9910, t9916)
}
