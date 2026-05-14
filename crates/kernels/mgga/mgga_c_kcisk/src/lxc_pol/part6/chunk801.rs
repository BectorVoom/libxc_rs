//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 801/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk801<F: Float>(t11443: F, t28414: F, t706: F, t11417: F, t11418: F, t28368: F, t2488: F, t8536: F, t7055: F, t1876: F, t4598: F, t11328: F, t4595: F, t1421: F, t16885: F, t16897: F, t22646: F, t22652: F, t22654: F, t22656: F, t456: F) -> (F, F, F) {
    let t28885 = t11443 * t28414;
    let t28886 = t706 * t28885;
    let t28894 = t11417 * t11418 * t28368;
    let t28897 = t2488 * t8536;
    let t28898 = t7055 * t28897;
    let t28902 = t1876 * t4598 * t28368;
    let t28906 = t4595 * t11328 * t28368;
    let t28909 = -0.98556445e-3 * t16885 - 0.43802864444444444445e-3 * t16897 - 0.36958666875e-3 * t456 * t28886 + 0.21901432222222222222e-2 * t22646 - 0.26281718666666666667e-2 * t22652 + 0.13140859333333333334e-2 * t22654 - 0.59133867e-2 * t22656 + 0.29201909629629629629e-2 * t1421 * t28894 + 0.59133867e-2 * t1421 * t28898 + 0.39422577999999999999e-2 * t1421 * t28902 - 0.65704296666666666666e-2 * t1421 * t28906;
    (t28885, t28897, t28909)
}
