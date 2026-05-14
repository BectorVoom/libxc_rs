//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1272/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1272<F: Float>(t32401: F, t32417: F, t109579: F, t9516: F, t32001: F, t3748: F, t13917: F, t32349: F, t9536: F, t1413: F, t394: F, t382: F, t123: F, t2734: F, t32393: F, t1333: F, t32150: F) -> (F, F, F, F, F, F, F, F) {
    let t109856 = t32417 * t32401;
    let t109858 = t9516 * t109579;
    let t109875 = t3748 * t32001;
    let t109880 = t9536 * t13917 * t32349;
    let t109882 = t1413 * t394;
    let t109883 = t109882 * t382;
    let t109888 = t2734 * t32393 * t123;
    let t109891 = t1333 * t32150;
    (t109856, t109858, t109875, t109880, t109882, t109883, t109888, t109891)
}
