//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1023/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1023<F: Float>(t1714: F, t23625: F, t1707: F, t23460: F, t23587: F, t23590: F, t23593: F, t23596: F, t23599: F, t23602: F, t23606: F, t23609: F, t11013: F, t17375: F, t17379: F, t17382: F, t17385: F, t23466: F, t23472: F, t23481: F, t23484: F, t23487: F, t23490: F, t23539: F, t23542: F, t23545: F, t23547: F, t23550: F, t23565: F, t23570: F, t23576: F, t23579: F, t23583: F) -> (F, F, F) {
    let t23626 = t1714 * t23625;
    let t23628 = t1707 * t23625;
    let t23630 = -0.73586666666666666666e-1 * t23587 + 0.22076e0 * t23590 - 0.99342e0 * t23593 - 0.132456e1 * t23596 - 0.5519e-1 * t23599 - 0.16557e0 * t23602 + 0.67094444444444444443e-1 * t23460 + 0.36793333333333333333e-1 * t23606 + 0.11038e0 * t23609 + 0.16504875e0 * t23626 + 0.258925e1 * t23628;
    let t23632 = -0.258925e1 * t23539 - 0.1294625e1 * t23542 - 0.412621875e-1 * t23545 + 0.16504875e0 * t23547 + 0.82524375e-1 * t23550 - 0.18396666666666666667e0 * t11013 - 0.44152e0 * t17375 - 0.40256666666666666668e0 * t17379 - 0.26837777777777777779e0 * t17382 - 0.36793333333333333333e0 * t17385 + t23565 + 0.60385e0 * t23487 - 0.20128333333333333333e0 * t23484 - 0.22076e0 * t23570 - 0.20128333333333333333e0 * t23472 + 0.10064166666666666667e0 * t23481 - 0.301925e0 * t23490 + 0.19419375e1 * t23576 + 0.33114e0 * t23579 + 0.12077e1 * t23466 + 0.33114e0 * t23583 + t23630;
    (t23626, t23628, t23632)
}
