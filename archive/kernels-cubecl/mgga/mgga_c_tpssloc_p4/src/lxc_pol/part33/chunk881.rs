//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 881/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk881<F: Float>(t2932: F, t5790: F, t2844: F, t5726: F, t2888: F, t5758: F, t225: F, t5849: F, t5851: F, t1040: F, t5904: F, t248: F, t3101: F, t5867: F) -> (F, F, F, F, F, F, F) {
    let t17492 = t5790 * t2932;
    let t17520 = t5726 * t2844;
    let t17547 = t5758 * t2888;
    let t17575 = t5849 * t225;
    let t17588 = t5851 * t225;
    let t17607 = t5904 * t1040;
    let t17611 = t248 * t3101 * t5867;
    (t17492, t17520, t17547, t17575, t17588, t17607, t17611)
}
