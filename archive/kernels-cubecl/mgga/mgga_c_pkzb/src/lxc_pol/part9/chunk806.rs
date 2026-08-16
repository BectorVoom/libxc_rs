//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 806/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk806<F: Float>(t5519: F, t5522: F, t5525: F, t5539: F, t261: F, t237: F, t1847: F, t663: F, t685: F, t1850: F, t1894: F, t1898: F, t659: F) -> (F, F, F, F, F, F, F, F) {
    let t5758 = F::cast_from(0.28842592592592592592e-1_f64) * t5519;
    let t5762 = -t5758 + F::cast_from(0.37083333333333333334e-1_f64) * t5522 - F::cast_from(0.278125e-1_f64) * t5525 + F::cast_from(0.278125e-1_f64) * t5539;
    let t5763 = t5762 * t261;
    let t5765 = F::cast_from(0.19751673498613801407e-1_f64) * t237 * t5763;
    let t5766 = t1847 * t663;
    let t5768 = F::cast_from(3.0_f64) * t5766 * t685;
    let t5770 = F::cast_from(3.0_f64) * t1850 * t1894;
    let t5771 = t659 * t1898;
    (t5758, t5762, t5763, t5765, t5766, t5768, t5770, t5771)
}
