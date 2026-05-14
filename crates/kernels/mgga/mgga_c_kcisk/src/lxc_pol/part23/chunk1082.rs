//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1082/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1082<F: Float>(t2293: F, t4437: F, t2297: F, t3697: F, t4455: F, t1203: F, t5761: F, t1543: F, t19478: F, t19480: F, t19482: F, t19559: F, t19582: F, t19585: F, t19587: F, t21730: F, t4436: F, t4461: F, t4478: F, t516: F) -> (F,) {
    let t21733 = t2293 * t4437;
    let t21736 = t2297 * t3697;
    let t21739 = t2293 * t4455;
    let t21742 = t5761 * t1203;
    let t21745 = -0.3109e-1 * t21730 * t516 + 6.0 * t4461 * t21733 + 0.35089340384731224426e1 * t4478 * t21736 - 2.0 * t4436 * t21739 - t19478 - t19480 - t19482 - t19559 + t19582 - t19585 - t19587 + 0.11696446794910408142e1 * t21742 * t1543;
    (t21745,)
}
