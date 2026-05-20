//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3192/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3192<F: Float>(t21063: F, t5362: F, t17308: F, t20846: F, t24639: F, t3172: F, t3711: F, t13062: F, t24545: F, t1122: F, t12855: F, t12910: F, t17736: F, t20858: F, t21119: F, t24713: F, t24751: F, t3626: F, t3720: F, t44704: F, t57147: F, t58824: F, t6688: F, t71055: F, t71117: F) -> F {
    let t83849 = t21063 * t5362;
    let t83851 = t17308 * t20846;
    let t83860 = t3711 * t3172 * t24639;
    let t83863 = t13062 * t3172 * t24545;
    let t83865 = F::cast_from(0.25724410870841842184e-2_f64) * t12910 * t3720 * t6688 * t71055 + F::cast_from(0.19055119163586549765e-3_f64) * t58824 - F::cast_from(0.68598428988911579154e-2_f64) * t57147 * t20858 - F::cast_from(0.85748036236139473947e-3_f64) * t17736 * t3626 * t24713 * t1122 + F::cast_from(0.45732285992607719436e-2_f64) * t83849 + F::cast_from(0.85748036236139473947e-3_f64) * t83851 - F::cast_from(0.42344709252414555034e-4_f64) * t44704 - F::cast_from(0.12862205435420921092e-2_f64) * t12855 * t3720 * t24751 * t21119 + F::cast_from(0.28582678745379824648e-3_f64) * t71117 - F::cast_from(0.47637797908966374413e-3_f64) * t83860 + F::cast_from(0.14291339372689912324e-3_f64) * t83863;
    t83865
}
