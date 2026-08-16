//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 616/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk616<F: Float>(t2792: F, t5695: F, t1547: F, t2798: F, t2802: F, t4335: F, t5679: F, t5683: F, t5687: F, t894: F, t2815: F, t901: F) -> (F, F, F, F, F, F, F) {
    let t5697 = F::cast_from(2.0_f64) * t2792 * t5695;
    let t5698 = t1547 * t1547;
    let t5699 = t2798 * t5698;
    let t5705 = t2802 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t4335 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t5679 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t5683 - t5687 / F::cast_from(3.0_f64);
    let t5706 = t894 * t5705;
    let t5712 = t2815 * t5698;
    let t5714 = t901 * t5705;
    (t5697, t5698, t5699, t5705, t5706, t5712, t5714)
}
