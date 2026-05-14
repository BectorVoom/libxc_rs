//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 988/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk988<F: Float>(t708: F, t8522: F, t1648: F, t7028: F, t682: F, t1824: F, t4629: F, t8504: F, t16887: F, t1417: F, t8928: F, t1882: F, t22596: F, t706: F, t1421: F, t16945: F, t16957: F, t22643: F, t22646: F, t22649: F, t22652: F, t22654: F, t22656: F, t22660: F, t22664: F, t22669: F, t22673: F, t22919: F, t2399: F, t456: F, t604: F, t6884: F) -> (F, F, F, F, F, F) {
    let t22922 = t708 * t8522;
    let t22923 = t22922 * t1648;
    let t22924 = t7028 * t22923;
    let t22927 = t682 * t8522;
    let t22928 = t22927 * t1824;
    let t22929 = t4629 * t22928;
    let t22932 = t708 * t8504;
    let t22933 = t22932 * t1648;
    let t22934 = t16887 * t22933;
    let t22937 = t682 * t8504;
    let t22938 = t22937 * t1824;
    let t22939 = t7028 * t22938;
    let t22942 = t1417 * t8928;
    let t22944 = t1882 * t22596;
    let t22945 = t706 * t22944;
    let t22950 = 0.39422578e-2 * t1421 * t22643 + 0.73004774074074074073e-3 * t22646 + 0.65704296666666666667e-3 * t1421 * t22649 - 0.87605728888888888887e-3 * t22652 + 0.43802864444444444445e-3 * t22654 - 0.19711289e-2 * t22656 - t16945 - 0.1478346675e-2 * t1421 * t22660 + 0.19711289e-2 * t1421 * t22664 + 0.295669335e-2 * t1421 * t22669 - 0.59133867e-2 * t1421 * t22673 + t16957 - 4.0 * t604 * t22919 + 0.7391733375e-3 * t1421 * t22924 - 0.1478346675e-2 * t1421 * t22929 - 0.36958666875e-3 * t1421 * t22934 - 0.7391733375e-3 * t1421 * t22939 + 0.13140859333333333333e-2 * t22942 + 0.1478346675e-2 * t456 * t22945 - 8.0 * t2399 * t6884;
    (t22923, t22928, t22933, t22938, t22944, t22950)
}
