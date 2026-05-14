//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 997/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk997<F: Float>(t13710: F, t13714: F, t13723: F, t13732: F, t13767: F, t13942: F, t13945: F, t13949: F, t14015: F, t9851: F, t9852: F, t13717: F, t13742: F, t13772: F, t13775: F, t13777: F, t13881: F, t13886: F, t13888: F, t13892: F, t13912: F, t13915: F, t13918: F, t13921: F, t13924: F, t13927: F, t13931: F, t13934: F, t14002: F, t9681: F, t9683: F, t9691: F) -> (F,) {
    let t14024 = t14015 - 0.40256666666666666667e0 * t13714 + 0.12077e1 * t13723 - 0.181155e1 * t13732 - t9851 - t9852 + 0.16504875e0 * t13942 + 0.258925e1 * t13767 - 0.91983333333333333334e-1 * t13945 - 0.13418888888888888889e0 * t13710 + 0.71747e0 * t13949;
    let t14026 = 0.19419375e1 * t13772 - 0.412621875e-1 * t13881 - 0.258925e1 * t13775 - 0.1294625e1 * t13777 + 0.16504875e0 * t13886 + 0.82524375e-1 * t13888 - 0.16557e0 * t13892 + 0.10064166666666666667e0 * t9681 + 0.67094444444444444447e-1 * t9683 - 0.26837777777777777778e0 * t9691 + t14002 + 0.36793333333333333334e-1 * t13912 - 0.27595e-1 * t13915 - 0.36793333333333333333e-1 * t13918 - 0.11038e0 * t13921 + 0.16557e0 * t13924 + 0.66228e0 * t13927 + 0.22141166666666666666e1 * t13717 + 0.16557e0 * t13931 - 0.49671e0 * t13934 - 0.60385e0 * t13742 + t14024;
    (t14026,)
}
