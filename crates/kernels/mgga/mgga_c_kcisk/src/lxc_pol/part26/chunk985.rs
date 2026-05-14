//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 985/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk985<F: Float>(t3521: F, t7846: F, t1422: F, t1423: F, t25312: F, t459: F, t7757: F, t1175: F, t19228: F, t425: F, t1364: F, t5926: F, t5684: F, t5927: F, t1421: F, t26520: F, t26524: F, t26528: F, t26532: F, t26536: F, t26540: F, t26544: F, t26547: F, t26550: F, t26555: F, t26560: F, t26564: F, t26569: F, t26574: F, t26577: F, t5913: F) -> (F, F, F, F) {
    let t26579 = t3521 * t7846;
    let t26582 = t1422 * t1423 * t25312;
    let t26585 = t459 * t7757;
    let t26586 = t26585 * t1175;
    let t26587 = t19228 * t26586;
    let t26590 = t425 * t7757;
    let t26591 = t26590 * t1364;
    let t26592 = t5926 * t26591;
    let t26595 = t5927 * t5684;
    let t26596 = t5926 * t26595;
    let t26599 = 0.43802864444444444444e-2 * t5913 * t26520 + 0.19711289e-2 * t1421 * t26524 + 0.39422578e-2 * t5913 * t26528 - 0.13140859333333333333e-2 * t1421 * t26532 - 0.26281718666666666666e-2 * t5913 * t26536 + 0.16426074166666666666e-2 * t1421 * t26540 - 0.10950716111111111111e-2 * t1421 * t26544 - 0.65704296666666666666e-2 * t1421 * t26547 + 0.29201909629629629629e-2 * t1421 * t26550 + 0.98556445e-3 * t1421 * t26555 + 0.13140859333333333333e-2 * t1421 * t26560 + 0.39422578e-2 * t1421 * t26564 + 0.7391733375e-3 * t1421 * t26569 - 0.1478346675e-2 * t1421 * t26574 - 0.87605728888888888887e-3 * t26577 + 0.73004774074074074073e-3 * t26579 + 0.65704296666666666667e-3 * t1421 * t26582 - 0.36958666875e-3 * t1421 * t26587 - 0.7391733375e-3 * t1421 * t26592 + 0.1478346675e-2 * t1421 * t26596;
    (t26586, t26591, t26595, t26599)
}
