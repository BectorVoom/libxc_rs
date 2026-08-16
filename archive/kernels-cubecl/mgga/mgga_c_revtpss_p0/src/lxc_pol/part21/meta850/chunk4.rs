//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3196/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3196<F: Float>(t3721: F, t44799: F, t12948: F, t17377: F, t17361: F, t3708: F, t17290: F, t3678: F, t1266: F, t12866: F, t12920: F, t12931: F, t1469: F, t17254: F, t17261: F, t17736: F, t17737: F, t21035: F, t3626: F, t372: F, t44704: F, t44711: F, t44726: F, t44729: F, t44748: F, t44751: F, t44773: F, t44776: F, t5302: F, t58851: F, t58853: F, t58863: F, t58868: F) -> (F, F) {
    let t58872 = t3721 * t44799;
    let t58878 = t17377 * t12948;
    let t58882 = t3708 * t17361;
    let t58883 = F::cast_from(0.14291339372689912324e-3_f64) * t58882;
    let t58884 = t17290 * t3678;
    let t58886 = -F::cast_from(0.1270341277572436651e-3_f64) * t44704 + F::cast_from(0.25724410870841842183e-2_f64) * t17261 * t17254 + F::cast_from(0.28582678745379824648e-3_f64) * t44711 - F::cast_from(0.28582678745379824648e-3_f64) * t44726 - F::cast_from(0.57165357490759649295e-3_f64) * t44729 + t58851 - F::cast_from(0.42874018118069736972e-3_f64) * t58853 - F::cast_from(0.85748036236139473944e-3_f64) * t17736 * t3626 * t17737 * t12931 - F::cast_from(0.17149607247227894789e-2_f64) * t17736 * t3626 * t21035 * t12920 + F::cast_from(0.22866142996303859718e-2_f64) * t58863 * t1266 + F::cast_from(0.85748036236139473944e-3_f64) * t58868 - F::cast_from(0.14291339372689912324e-2_f64) * t12866 * t372 * t5302 * t1469 * t58872 - F::cast_from(0.85748036236139473944e-3_f64) * t44748 - F::cast_from(0.19055119163586549765e-3_f64) * t44751 - F::cast_from(0.42874018118069736972e-3_f64) * t58878 - F::cast_from(0.42874018118069736972e-3_f64) * t44773 + F::cast_from(0.85748036236139473944e-3_f64) * t44776 - t58883 - F::cast_from(0.85748036236139473944e-3_f64) * t58884;
    (t58872, t58886)
}
