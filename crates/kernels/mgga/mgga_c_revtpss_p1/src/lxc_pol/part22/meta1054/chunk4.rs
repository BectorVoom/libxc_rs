//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3729/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3729<F: Float>(t21177: F, t3678: F, t17303: F, t5327: F, t1250: F, t12809: F, t13099: F, t16715: F, t16738: F, t16742: F, t17212: F, t17353: F, t17426: F, t17693: F, t17732: F, t17737: F, t17742: F, t17781: F, t17784: F, t1794: F, t20795: F, t20800: F, t20802: F, t20929: F, t21017: F, t3626: F, t372: F, t3720: F, t44561: F, t5331: F, t57265: F, t57534: F, t70647: F) -> F {
    let t70756 = t21177 * t3678;
    let t70758 = t5327 * t17303;
    let t70789 = -F::cast_from(0.1270341277572436651e-2_f64) * t17693 * t372 * t13099 * t1794 * t1250 * t16715 - F::cast_from(0.96545937095505185476e-2_f64) * t70756 + F::cast_from(0.95275595817932748827e-4_f64) * t70758 - F::cast_from(0.11433071498151929859e-2_f64) * t17693 * t17353 * t1250 * t16738 - F::cast_from(0.57165357490759649296e-3_f64) * t17693 * t17353 * t1250 * t16742 + F::cast_from(0.85748036236139473944e-3_f64) * t17426 * t20802 + F::cast_from(0.11433071498151929859e-2_f64) * t57534 + F::cast_from(0.17149607247227894789e-2_f64) * t57265 * t3626 * t17737 * t17212 + F::cast_from(0.21437009059034868486e-3_f64) * t12809 * t3720 * t20795 * t17742 + F::cast_from(0.45732285992607719436e-2_f64) * t21017 * t17781 - F::cast_from(0.60976381323476959248e-2_f64) * t70647 * t17732 + F::cast_from(0.57165357490759649296e-3_f64) * t44561 * t20929 - F::cast_from(0.21437009059034868486e-3_f64) * t5331 * t3720 * t20800 * t17784;
    t70789
}
