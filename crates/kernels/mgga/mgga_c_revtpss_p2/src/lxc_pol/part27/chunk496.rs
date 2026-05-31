//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 496/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk496<F: Float>(t125: F, t2749: F, t836: F, t2747: F, t231: F, t2722: F, t827: F, t828: F, t2695: F, t2702: F, t2704: F, t2707: F, t2716: F, t2721: F, t2726: F, t2730: F, t2732: F, t2739: F, t2742: F, t2745: F, t799: F, t825: F) -> (F, F, F, F) {
    let t2750 = t125 * t836 * t2749;
    let t2751 = t2747 * t2750;
    let t2754 = t2722 * t231;
    let t2756 = t827 * t828 * t2754;
    let t2759 = F::cast_from(0.57165357490759649296e-4_f64) * t2695 + t2702 + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t2704 - t799 * t2707 / F::cast_from(48.0_f64) + t2716 + F::cast_from(0.42874018118069736972e-3_f64) * t2721 * t2726 + t2730 * t2732 / F::cast_from(16.0_f64) - t2739 + F::cast_from(0.20007875121765877254e-2_f64) * t2742 + F::cast_from(0.17149607247227894789e-2_f64) * t2745 * t2751 - F::cast_from(0.21437009059034868486e-3_f64) * t825 * t2756;
    (t2751, t2754, t2756, t2759)
}
