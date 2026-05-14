//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 954/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk954<F: Float>(t3521: F, t7036: F, t11250: F, t11340: F, t11342: F, t11344: F, t11350: F, t11390: F, t1421: F, t16855: F, t16859: F, t16863: F, t16865: F, t16867: F, t16873: F, t16879: F, t16882: F, t16885: F, t16889: F, t16894: F, t16897: F, t16900: F, t2399: F, t456: F, t4794: F) -> (F,) {
    let t16902 = 0.19711289e-2 * t3521 * t7036;
    let t16903 = 0.492782225e-3 * t11340 + 0.98556445e-3 * t11342 + 0.1478346675e-2 * t456 * t16855 + 0.39422578e-2 * t1421 * t16859 - t16863 + t16865 - 0.1478346675e-2 * t1421 * t16867 + 0.43802864444444444444e-3 * t11344 - 0.65704296666666666666e-3 * t11350 - 0.98556445e-3 * t456 * t16873 + 0.13140859333333333333e-2 * t11390 - 4.0 * t4794 * t2399 + 0.21901432222222222222e-3 * t16879 - 0.13140859333333333333e-2 * t11250 * t16882 - 0.32852148333333333333e-3 * t16885 - 0.36958666875e-3 * t1421 * t16889 + 0.295669335e-2 * t1421 * t16894 - 0.14600954814814814815e-3 * t16897 + t16900 - t16902;
    (t16903,)
}
