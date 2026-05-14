//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1102/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1102<F: Float>(t32955: F, t9664: F, t4648: F, t9665: F, t1775: F, t11204: F, t2781: F, t1863: F, t1871: F, t1895: F, t415: F, t1864: F, t1900: F, t2785: F, t32925: F, t32928: F, t32932: F, t32938: F, t32942: F, t32948: F, t32952: F, t9652: F, t9672: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t32956 = t9664 * t32955;
    let t32958 = t9665 * t4648;
    let t32959 = t1775 * t32958;
    let t32962 = t11204 * t2781;
    let t32965 = t1863 * t1871;
    let t32966 = t32965 * t1895;
    let t32967 = t415 * t32966;
    let t32969 = t1864 * t1900;
    let t32970 = t415 * t32969;
    let t32972 = 0.22109259259259259258e-2 * t32925 + 0.16581944444444444444e-2 * t32928 - 0.10416666666666666667e-1 * t32932 * t2785 - 0.20833333333333333334e-1 * t9664 * t32938 + 0.20833333333333333334e-1 * t32942 * t9672 + 0.20833333333333333334e-1 * t32942 * t9652 + 0.8041666666666666667e-2 * t32948 * t9652 - 0.46296296296296296297e-2 * t9664 * t32952 - 0.23148148148148148148e-2 * t32956 - 0.34722222222222222223e-2 * t9664 * t32959 - 0.10416666666666666667e-1 * t32962 * t2785 - 0.49745833333333333332e-2 * t32967 + 0.33163888888888888888e-2 * t32970;
    (t32956, t32958, t32959, t32962, t32965, t32966, t32967, t32969, t32970, t32972)
}
