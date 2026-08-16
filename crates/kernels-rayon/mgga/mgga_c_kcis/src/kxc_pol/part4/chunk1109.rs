//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1109/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1109(t13712: f64, t13710: f64, t13714: f64, t13723: f64, t13732: f64, t13767: f64, t13942: f64, t13945: f64, t13949: f64, t9851: f64, t9852: f64, t13717: f64, t13742: f64, t13772: f64, t13775: f64, t13777: f64, t13881: f64, t13886: f64, t13888: f64, t13892: f64, t13912: f64, t13915: f64, t13918: f64, t13921: f64, t13924: f64, t13927: f64, t13931: f64, t13934: f64, t14002: f64, t9681: f64, t9683: f64, t9691: f64) -> f64 {
    let t14015 = 0.13418888888888888889e0_f64 * t13712;
    let t14024 = t14015 - 0.40256666666666666667e0_f64 * t13714 + 0.12077e1_f64 * t13723 - 0.181155e1_f64 * t13732 - t9851 - t9852 + 0.16504875e0_f64 * t13942 + 0.258925e1_f64 * t13767 - 0.91983333333333333334e-1_f64 * t13945 - 0.13418888888888888889e0_f64 * t13710 + 0.71747e0_f64 * t13949;
    let t14026 = 0.19419375e1_f64 * t13772 - 0.412621875e-1_f64 * t13881 - 0.258925e1_f64 * t13775 - 0.1294625e1_f64 * t13777 + 0.16504875e0_f64 * t13886 + 0.82524375e-1_f64 * t13888 - 0.16557e0_f64 * t13892 + 0.10064166666666666667e0_f64 * t9681 + 0.67094444444444444447e-1_f64 * t9683 - 0.26837777777777777778e0_f64 * t9691 + t14002 + 0.36793333333333333334e-1_f64 * t13912 - 0.27595e-1_f64 * t13915 - 0.36793333333333333333e-1_f64 * t13918 - 0.11038e0_f64 * t13921 + 0.16557e0_f64 * t13924 + 0.66228e0_f64 * t13927 + 0.22141166666666666666e1_f64 * t13717 + 0.16557e0_f64 * t13931 - 0.49671e0_f64 * t13934 - 0.60385e0_f64 * t13742 + t14024;
    t14026
}
