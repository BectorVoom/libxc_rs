//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 539/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk539<F: Float>(t1685: F, t4761: F, t4762: F, t4636: F, t4722: F, t4638: F, t4642: F, t4646: F, t4650: F, t4672: F, t4674: F, t4717: F, t4719: F, t4724: F, t4728: F, t4731: F, t4734: F) -> (F, F) {
    let t4764 = t4761 * t4762 * t1685;
    let t4769 = 0.40256666666666666667e0 * t4636;
    let t4776 = 0.137975e0 * t4722;
    let t4781 = -0.1294625e1 * t4672 + 0.258925e1 * t4674 + t4769 + 0.20128333333333333334e0 * t4638 - 0.20128333333333333333e0 * t4642 + 0.60385e0 * t4646 - 0.301925e0 * t4650 + 0.82524375e-1 * t4717 + 0.16504875e0 * t4719 + t4776 + 0.11038e0 * t4724 - 0.27595e-1 * t4728 + 0.16557e0 * t4731 - 0.82785e-1 * t4734;
    (t4764, t4781)
}
