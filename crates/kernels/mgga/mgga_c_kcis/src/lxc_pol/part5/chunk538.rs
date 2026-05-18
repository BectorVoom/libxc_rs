//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 538/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk538<F: Float>(t206: F, t882: F, t209: F, t2739: F, t2718: F, t880: F, t208: F, t214: F, t2733: F, t876: F, t884: F, t888: F, t221: F, t2423: F, t2427: F, t2430: F, t2482: F, t2486: F, t2494: F, t2529: F, t2720: F, t2725: F, t2729: F, t874: F, t889: F) -> (F, F, F, F, F, F, F) {
    let t210 = F::new(0.0) < t206;
    let t2740 = t882 * t882;
    let t2742 = t209 * t2739 * t2740;
    let t2746 = piecewise3::<f64>(t210, t2718, -t2718);
    let t2748 = t209 * t880 * t2746;
    let t2751 = F::new(35.0) / F::new(432.0) * t2733 * t214 + F::new(7.0) / F::new(144.0) * t876 * t884 + t208 * t2742 / F::new(48.0) - t208 * t2748 / F::new(96.0);
    let t2752 = t2751 * t888;
    let t2764 = t2718 * t221 - F::new(0.13345e0) * t2720 * t889 + F::new(0.890445125e-2) * t2725 * t2729 - F::new(0.66725e-1) * t874 * t2752 + F::new(0.66725e-1) * t874 * t2729 + F::new(0.21667074074074074073e-1) * t2423 - F::new(0.18571777777777777777e-1) * t2427 + F::new(0.18571777777777777777e-1) * t2430 + F::new(0.69644166666666666665e-2) * t2482 - F::new(0.13928833333333333333e-1) * t2486 + F::new(0.13928833333333333333e-1) * t2494 - F::new(0.69644166666666666665e-2) * t2529;
    (t2740, t2742, t2746, t2748, t2751, t2752, t2764)
}
