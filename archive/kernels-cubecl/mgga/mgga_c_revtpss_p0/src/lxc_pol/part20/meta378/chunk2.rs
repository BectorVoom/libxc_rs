//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1372/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1372<F: Float>(t2682: F, t820: F, t823: F, t2751: F, t10764: F, t10797: F, t10870: F, t14547: F, t14785: F, t14894: F, t2721: F, t2724: F, t2745: F, t2747: F, t2749: F, t40263: F, t40523: F, t40526: F, t40529: F, t40532: F, t40535: F, t40537: F, t40549: F, t40553: F, t40558: F, t40560: F, t40569: F, t40581: F, t40586: F, t4362: F, t4364: F, t827: F, t828: F, t837: F) -> F {
    let t40593 = t820 * t823 * t2682;
    let t40594 = t40593 * t2751;
    let t40596 = -F::cast_from(0.30492001685571196934e-4_f64) * t40523 - F::cast_from(0.1084295579938911763e-3_f64) * t40526 + F::cast_from(0.15246000842785598467e-4_f64) * t40529 + F::cast_from(0.5421477899694558815e-3_f64) * t40532 + F::cast_from(0.13011546959266941156e-2_f64) * t40535 - F::cast_from(0.77173232612525526552e-2_f64) * t10870 * t827 * t828 * t40537 + F::cast_from(0.30011812682648815881e-2_f64) * t2721 * t827 * t828 * t40263 + F::cast_from(0.17149607247227894789e-2_f64) * t40549 - F::cast_from(0.34299214494455789577e-3_f64) * t40553 - F::cast_from(0.34299214494455789577e-3_f64) * t40558 - F::cast_from(0.51448821741683684368e-1_f64) * t2745 * t14785 * t40560 * t2749 - F::cast_from(0.77173232612525526552e-2_f64) * t14894 * t4364 * t10797 * t14547 + F::cast_from(0.34299214494455789577e-2_f64) * t2745 * t2747 * t40569 * t2749 - F::cast_from(0.10289764348336736873e-1_f64) * t4362 * t2747 * t10764 * t2724 + F::cast_from(0.30492001685571196935e-3_f64) * t40581 + F::cast_from(0.60984003371142393869e-3_f64) * t40586 - F::cast_from(0.85748036236139473944e-3_f64) * t2745 * t4364 * t40569 * t837 + F::cast_from(0.27210710165601593065e0_f64) * t40594;
    t40596
}
