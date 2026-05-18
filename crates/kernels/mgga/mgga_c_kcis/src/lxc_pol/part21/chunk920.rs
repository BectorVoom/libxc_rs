//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 920/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk920<F: Float>(t13712: F, t13710: F, t13714: F, t13723: F, t13732: F, t13767: F, t13942: F, t13945: F, t13949: F, t9851: F, t9852: F, t13717: F, t13742: F, t13772: F, t13775: F, t13777: F, t13881: F, t13886: F, t13888: F, t13892: F, t13912: F, t13915: F, t13918: F, t13921: F, t13924: F, t13927: F, t13931: F, t13934: F, t14002: F, t9681: F, t9683: F, t9691: F) -> F {
    let t14015 = F::new(0.13418888888888888889e0) * t13712;
    let t14024 = t14015 - F::new(0.40256666666666666667e0) * t13714 + F::new(0.12077e1) * t13723 - F::new(0.181155e1) * t13732 - t9851 - t9852 + F::new(0.16504875e0) * t13942 + F::new(0.258925e1) * t13767 - F::new(0.91983333333333333334e-1) * t13945 - F::new(0.13418888888888888889e0) * t13710 + F::new(0.71747e0) * t13949;
    let t14026 = F::new(0.19419375e1) * t13772 - F::new(0.412621875e-1) * t13881 - F::new(0.258925e1) * t13775 - F::new(0.1294625e1) * t13777 + F::new(0.16504875e0) * t13886 + F::new(0.82524375e-1) * t13888 - F::new(0.16557e0) * t13892 + F::new(0.10064166666666666667e0) * t9681 + F::new(0.67094444444444444447e-1) * t9683 - F::new(0.26837777777777777778e0) * t9691 + t14002 + F::new(0.36793333333333333334e-1) * t13912 - F::new(0.27595e-1) * t13915 - F::new(0.36793333333333333333e-1) * t13918 - F::new(0.11038e0) * t13921 + F::new(0.16557e0) * t13924 + F::new(0.66228e0) * t13927 + F::new(0.22141166666666666666e1) * t13717 + F::new(0.16557e0) * t13931 - F::new(0.49671e0) * t13934 - F::new(0.60385e0) * t13742 + t14024;
    t14026
}
