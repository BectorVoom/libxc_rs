//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 743/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk743<F: Float>(t2754: F, t827: F, t828: F, t2695: F, t2702: F, t2704: F, t2707: F, t2716: F, t2721: F, t2726: F, t2730: F, t2732: F, t2739: F, t2742: F, t2745: F, t2751: F, t799: F, t825: F) -> (F, F) {
    let t2756 = t827 * t828 * t2754;
    let t2759 = F::cast_from(0.57165357490759649296e-4_f64) * t2695 + t2702 + F::new(7.0) / F::new(72.0) * t2704 - t799 * t2707 / F::new(48.0) + t2716 + F::cast_from(0.42874018118069736972e-3_f64) * t2721 * t2726 + t2730 * t2732 / F::new(16.0) - t2739 + F::cast_from(0.20007875121765877254e-2_f64) * t2742 + F::cast_from(0.17149607247227894789e-2_f64) * t2745 * t2751 - F::cast_from(0.21437009059034868486e-3_f64) * t825 * t2756;
    (t2756, t2759)
}
