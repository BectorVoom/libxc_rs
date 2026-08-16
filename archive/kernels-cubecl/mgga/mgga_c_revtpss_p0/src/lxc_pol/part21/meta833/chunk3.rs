//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3118/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3118<F: Float>(t12773: F, t17448: F, t12916: F, t17780: F, t5331: F, t1260: F, t45385: F, t12640: F, t17728: F, t489: F, t1042: F, t12257: F, t12712: F, t12781: F, t12832: F, t12872: F, t12956: F, t13045: F, t13099: F, t1715: F, t17347: F, t17351: F, t17584: F, t17602: F, t17605: F, t17688: F, t17709: F, t17710: F, t17739: F, t1774: F, t17747: F, t17753: F, t20945: F, t21049: F, t3603: F, t3626: F, t3711: F, t3720: F, t44501: F, t471: F, t57314: F, t57316: F, t57318: F, t57321: F, t57325: F, t57331: F) -> F {
    let t57333 = t17448 * t12773;
    let t57336 = t5331 * t12916 * t17780;
    let t57344 = t45385 * t1260;
    let t57348 = t12640 * t489 * t17728;
    let t57370 = F::cast_from(0.63517063878621832552e-3_f64) * t3711 * t1042 * t13099 * t1774 * t12257 - F::cast_from(0.45732285992607719436e-2_f64) * t57314 + F::cast_from(0.85748036236139473944e-3_f64) * t57316 - F::cast_from(0.45732285992607719436e-2_f64) * t57318 + F::cast_from(0.28582678745379824648e-2_f64) * t57321 + F::cast_from(0.42874018118069736972e-3_f64) * t12956 * t17584 + F::cast_from(0.64311027177104605458e-3_f64) * t17753 * t3720 * t17710 * t57325 + F::cast_from(0.95275595817932748826e-4_f64) * t57331 - F::cast_from(0.57165357490759649295e-3_f64) * t57333 - F::cast_from(0.85748036236139473944e-3_f64) * t57336 - F::cast_from(0.64311027177104605458e-3_f64) * t12832 * t17602 - F::cast_from(0.7145669686344956162e-3_f64) * t17351 * t20945 * t12712 * t17688 - F::cast_from(0.38586616306262763275e-2_f64) * t57344 * t17347 - F::cast_from(0.17149607247227894789e-2_f64) * t57348 * t17739 + F::cast_from(0.12862205435420921092e-2_f64) * t21049 * t12872 + F::cast_from(0.45732285992607719436e-2_f64) * t17605 * t12781 - F::cast_from(0.85748036236139473944e-3_f64) * t17709 * t3626 * t1715 * t44501 * t13045 + F::cast_from(0.85748036236139473944e-3_f64) * t17747 * t3626 * t1715 * t44501 * t3603 - F::cast_from(0.14291339372689912324e-3_f64) * t17753 * t3626 * t1715 * t44501 * t471;
    t57370
}
