//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 466/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk466<F: Float>(t4663: F, t677: F, t4636: F, t1643: F, t583: F, t573: F, t571: F, t1379: F, t311: F, t579: F, t1774: F, t79: F, t586: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4664 = t4663 * t677;
    let t4676 = 0.55033333333333333333e-2 * t4636;
    let t4691 = 0.23744444444444444444e-1 * t4636;
    let t4702 = t1643 * t583;
    let t4703 = 1.0 / t4702;
    let t4704 = t573 * t4703;
    let t4711 = 0.39862222222222222223e0 * t4636;
    let t4716 = 1.0/f64::sqrt(t571);
    let t4722 = t311 * t1379 * t579;
    let t4723 = 0.13692777777777777778e0 * t4722;
    let t4726 = t79 * t1774;
    let t4740 = t1643 * t1643;
    let t4741 = 1.0 / t4740;
    let t4742 = t573 * t4741;
    let t4743 = t586 * t586;
    let t4744 = 1.0 / t4743;
    let t4748 = 0.12361111111111111111e-1 * t4636;
    (t4664, t4676, t4691, t4703, t4704, t4711, t4716, t4722, t4723, t4726, t4740, t4741, t4742, t4743, t4744, t4748)
}
