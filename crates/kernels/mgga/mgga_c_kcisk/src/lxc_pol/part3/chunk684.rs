//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 684/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk684<F: Float>(t10660: F, t1653: F, t571: F, t1648: F, t4624: F, t1646: F, t574: F, t581: F, t4663: F, t4652: F, t4716: F, t10579: F, t10582: F, t10590: F, t10598: F, t10639: F, t10642: F, t10644: F, t10647: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10661 = t1653 * t10660;
    let t10663 = F::new(1.0)/pow_3_2::<F>(t571);
    let t10664 = t4624 * t1648;
    let t10665 = t10663 * t10664;
    let t10667 = t1646 * t10660;
    let t10671 = F::new(1.0) / t574 / t581 / F::new(4.0);
    let t10672 = t10671 * t10664;
    let t10674 = t4663 * t1648;
    let t10675 = t10674 * t4652;
    let t10677 = t4716 * t1648;
    let t10678 = t10677 * t4652;
    let t10680 = -F::cast_from(0.33547222222222222222e0_f64) * t10579 + F::new(0.12077e1) * t10582 - F::new(0.181155e1) * t10590 - F::new(0.301925e0) * t10598 - t10639 - t10642 - F::new(0.82785e-1) * t10644 + F::new(0.49671e0) * t10647 + F::new(0.16504875e0) * t10661 - F::cast_from(0.412621875e-1_f64) * t10665 + F::new(0.258925e1) * t10667 + F::new(0.19419375e1) * t10672 - F::new(0.3883875e1) * t10675 + F::cast_from(0.247573125e0_f64) * t10678;
    (t10661, t10664, t10665, t10667, t10671, t10672, t10675, t10678, t10680)
}
