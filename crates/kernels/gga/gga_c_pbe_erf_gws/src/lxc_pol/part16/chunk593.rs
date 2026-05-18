//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 593/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk593<F: Float>(t2685: F, t639: F, t1000: F, t610: F, t1827: F, t587: F, t1684: F, t1741: F, t1788: F, t1028: F, t395: F, t1691: F, t2679: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2687 = F::new(4.0) / F::new(45.0) * t639 * t2685;
    let t2688 = t1000 * t610;
    let t2689 = t1827 * t2688;
    let t2691 = F::new(4.0) / F::new(45.0) * t587 * t2689;
    let t2692 = F::new(4.0) / F::new(45.0) * t1684;
    let t2693 = F::new(4.0) / F::new(45.0) * t1741;
    let t2694 = F::new(4.0) / F::new(45.0) * t1788;
    let t2696 = t395 * t1028;
    let t2698 = t1691 * t2679;
    (t2687, t2688, t2689, t2691, t2692, t2693, t2694, t2696, t2698)
}
