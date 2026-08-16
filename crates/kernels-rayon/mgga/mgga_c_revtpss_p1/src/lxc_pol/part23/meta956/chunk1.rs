//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3192/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3192(t21063: f64, t5362: f64, t17308: f64, t20846: f64, t24639: f64, t3172: f64, t3711: f64, t13062: f64, t24545: f64, t1122: f64, t12855: f64, t12910: f64, t17736: f64, t20858: f64, t21119: f64, t24713: f64, t24751: f64, t3626: f64, t3720: f64, t44704: f64, t57147: f64, t58824: f64, t6688: f64, t71055: f64, t71117: f64) -> f64 {
    let t83849 = t21063 * t5362;
    let t83851 = t17308 * t20846;
    let t83860 = t3711 * t3172 * t24639;
    let t83863 = t13062 * t3172 * t24545;
    let t83865 = 0.25724410870841842184e-2_f64 * t12910 * t3720 * t6688 * t71055 + 0.19055119163586549765e-3_f64 * t58824 - 0.68598428988911579154e-2_f64 * t57147 * t20858 - 0.85748036236139473947e-3_f64 * t17736 * t3626 * t24713 * t1122 + 0.45732285992607719436e-2_f64 * t83849 + 0.85748036236139473947e-3_f64 * t83851 - 0.42344709252414555034e-4_f64 * t44704 - 0.12862205435420921092e-2_f64 * t12855 * t3720 * t24751 * t21119 + 0.28582678745379824648e-3_f64 * t71117 - 0.47637797908966374413e-3_f64 * t83860 + 0.14291339372689912324e-3_f64 * t83863;
    t83865
}
