//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 933/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk933<F: Float>(t13820: F, t389: F, t3970: F, t3984: F, t25: F, t3962: F, t1309: F, t1318: F, t398: F, t1322: F, t3961: F, t1310: F) -> (F, F, F, F) {
    let t13821 = t389 * t13820;
    let t13824 = t3970 * t3984;
    let t13826 = t25 * t3962;
    let t13827 = t1309 * t13826;
    let t13829 = t1318 * t1318;
    let t13830 = F::new(1.0) / t13829;
    let t13831 = t398 * t13830;
    let t13832 = t3961 * t1322;
    let t13833 = t13831 * t13832;
    let t13834 = t1310 * t13833;
    (t13821, t13824, t13827, t13834)
}
