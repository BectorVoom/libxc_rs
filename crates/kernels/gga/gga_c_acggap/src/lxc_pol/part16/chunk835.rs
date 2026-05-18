//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 835/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk835<F: Float>(t598: F, t9724: F, t142: F, t1866: F, t7436: F, t7815: F, t2030: F, t1782: F, t7351: F, t2060: F, t7718: F, t7726: F, t7738: F, t7740: F, t7743: F, t7748: F, t7776: F, t7782: F, t7788: F, t7801: F, t7803: F, t9335: F, t9713: F, t9715: F, t9717: F, t9721: F) -> (F, F, F, F, F) {
    let t9725 = t598 * t9724;
    let t9727 = t142 * t1866;
    let t9728 = t7436 * t9727;
    let t9730 = t7815 * t1866;
    let t9731 = t2030 * t9730;
    let t9733 = t7351 * t1782;
    let t9734 = t142 * t9733;
    let t9735 = t2060 * t9734;
    let t9737 = -F::new(0.4584375e-1) * t9713 - F::new(0.17149607247227894789e-2) * t9715 + F::new(0.17149607247227894789e-2) * t9717 + F::new(0.31448092289604152068e-2) * t9721 - t7718 - t7726 + t9335 + F::new(0.47172138434406228102e-3) * t9725 - t7738 - t7740 + t7743 + t7748 + t9728 / F::new(24.0) + t9731 / F::new(64.0) - F::new(0.22921875e-1) * t9735 - t7776 + t7782 - t7788 + t7801 - t7803;
    (t9727, t9730, t9733, t9734, t9737)
}
