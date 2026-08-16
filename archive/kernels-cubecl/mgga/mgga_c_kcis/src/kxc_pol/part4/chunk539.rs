//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 539/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk539<F: Float>(t206: F, t882: F, t209: F, t2739: F, t2718: F, t880: F, t208: F, t214: F, t2733: F, t876: F, t884: F, t888: F, t221: F, t2423: F, t2427: F, t2430: F, t2482: F, t2486: F, t2494: F, t2529: F, t2720: F, t2725: F, t2729: F, t874: F, t889: F) -> (F, F, F, F, F, F, F) {
    let t210 = F::cast_from(0.0_f64) < t206;
    let t2740 = t882 * t882;
    let t2742 = t209 * t2739 * t2740;
    let t2746 = piecewise3::<F>(t210, t2718, -t2718);
    let t2748 = t209 * t880 * t2746;
    let t2751 = F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t2733 * t214 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t876 * t884 + t208 * t2742 / F::cast_from(48.0_f64) - t208 * t2748 / F::cast_from(96.0_f64);
    let t2752 = t2751 * t888;
    let t2764 = t2718 * t221 - F::cast_from(0.13345e0_f64) * t2720 * t889 + F::cast_from(0.890445125e-2_f64) * t2725 * t2729 - F::cast_from(0.66725e-1_f64) * t874 * t2752 + F::cast_from(0.66725e-1_f64) * t874 * t2729 + F::cast_from(0.21667074074074074073e-1_f64) * t2423 - F::cast_from(0.18571777777777777777e-1_f64) * t2427 + F::cast_from(0.18571777777777777777e-1_f64) * t2430 + F::cast_from(0.69644166666666666665e-2_f64) * t2482 - F::cast_from(0.13928833333333333333e-1_f64) * t2486 + F::cast_from(0.13928833333333333333e-1_f64) * t2494 - F::cast_from(0.69644166666666666665e-2_f64) * t2529;
    (t2740, t2742, t2746, t2748, t2751, t2752, t2764)
}
