//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3145/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3145<F: Float>(t1266: F, t12866: F, t17261: F, t17649: F, t17693: F, t17799: F, t20932: F, t24605: F, t24647: F, t3604: F, t44510: F, t5405: F, t69721: F, t69773: F, t69839: F, t82579: F, t82587: F, t82591: F, t82595: F, t82597: F, t82603: F) -> F {
    let t82608 = F::cast_from(0.42874018118069736972e-3_f64) * t12866 * t17649 * t24647 * t5405 + F::cast_from(0.85748036236139473944e-3_f64) * t12866 * t17799 * t82579 + F::cast_from(0.85748036236139473944e-3_f64) * t44510 * t69839 * t3604 * t20932 - F::cast_from(0.85748036236139473944e-3_f64) * t17693 * t17799 * t82587 + F::cast_from(0.85748036236139473944e-3_f64) * t12866 * t17799 * t82591 + F::cast_from(0.30488190661738479624e-2_f64) * t82595 - F::cast_from(0.14481890564325777821e-1_f64) * t82597 * t1266 - F::cast_from(0.91464571985215438872e-2_f64) * t69721 - F::cast_from(0.95275595817932748827e-4_f64) * t82603 - F::cast_from(0.45732285992607719436e-2_f64) * t69773 - F::cast_from(0.85748036236139473944e-3_f64) * t17261 * t24605;
    t82608
}
