//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 747/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk747<F: Float>(t3739: F, t7836: F, t8083: F, t3748: F, t8086: F, t1333: F, t8164: F, t3924: F, t8059: F, t12841: F, t8094: F, t1219: F, t7828: F, t13959: F, t8177: F, t1458: F, t8185: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26869 = t3739 * t7836;
    let t26914 = t3739 * t8083;
    let t26919 = t3748 * t8086;
    let t26936 = t1333 * t8164;
    let t26992 = t8059 * t3924;
    let t27008 = t12841 * t8094;
    let t27016 = t7828 * t1219;
    let t27037 = t13959 * t8177;
    let t27047 = t8185 * t1458;
    (t26869, t26914, t26919, t26936, t26992, t27008, t27016, t27037, t27047)
}
