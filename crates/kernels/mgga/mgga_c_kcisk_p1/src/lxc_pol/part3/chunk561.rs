//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 561/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk561<F: Float>(t4663: F, t677: F, t1646: F, t1821: F, t4624: F, t4652: F, t4636: F, t4638: F, t4642: F, t4646: F, t4650: F, t1648: F, t1815: F, t574: F) -> (F, F, F, F, F, F) {
    let t4664 = t4663 * t677;
    let t4667 = t1646 * t1821;
    let t4672 = t4663 * t4624;
    let t4674 = t1646 * t4652;
    let t4676 = F::cast_from(0.55033333333333333333e-2_f64) * t4636;
    let t4681 = -F::cast_from(0.991e-2_f64) * t4672 + F::cast_from(0.1982e-1_f64) * t4674 + t4676 + F::cast_from(0.27516666666666666666e-2_f64) * t4638 - F::cast_from(0.27516666666666666667e-2_f64) * t4642 + F::cast_from(0.8255e-2_f64) * t4646 - F::cast_from(0.41275e-2_f64) * t4650;
    let t4684 = -t4664 * t4624 / F::cast_from(8.0_f64) + t4667 * t1648 / F::cast_from(2.0_f64) + t1815 * t4652 / F::cast_from(4.0_f64) + t574 * t4681 / F::cast_from(2.0_f64);
    (t4664, t4667, t4672, t4674, t4681, t4684)
}
