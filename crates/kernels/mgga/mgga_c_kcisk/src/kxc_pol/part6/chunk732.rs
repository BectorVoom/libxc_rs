//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 732/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk732<F: Float>(t1772: F, t24879: F, t7591: F, t7602: F, t7581: F, t4998: F, t9217: F, t2013: F, t9168: F, t10886: F, t9172: F, t2012: F, t23768: F, t2009: F, t9208: F, t9189: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24880 = t24879 * t1772;
    let t24908 = t7591 * t7602;
    let t24910 = t7581 * t7602;
    let t24912 = t4998 * t9217;
    let t24913 = t2013 * t24912;
    let t24920 = t4998 * t9168;
    let t24921 = t2013 * t24920;
    let t24925 = t10886 * t9172;
    let t24926 = t2013 * t24925;
    let t24967 = t2012 * t23768;
    let t24976 = t9208 * t2009;
    let t24978 = t9189 * t2009;
    (t24880, t24908, t24910, t24913, t24921, t24926, t24967, t24976, t24978)
}
