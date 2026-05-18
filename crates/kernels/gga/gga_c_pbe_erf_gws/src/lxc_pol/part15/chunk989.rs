//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 989/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk989<F: Float>(t3074: F, t8847: F, t814: F, t857: F, t858: F, t856: F, t6229: F, t2170: F, t2171: F, t8840: F, t2168: F, t8821: F, t8823: F, t8826: F, t8831: F, t8832: F, t8835: F, t8839: F, t8844: F, t8846: F) -> (F, F, F, F, F, F) {
    let t8848 = t3074 * t8847;
    let t8850 = t857 * t858 * t814;
    let t8851 = t856 * t8850;
    let t8853 = t8848 * t8851 / F::new(32.0);
    let t8854 = F::new(35.0) / F::new(216.0) * t6229;
    let t8856 = t2170 * t8840 * t2171;
    let t8858 = t2168 * t8856 / F::new(24.0);
    let t8859 = -t8821 + t8823 + t8826 + t8831 + t8832 + t8835 - t8839 + t8844 - t8846 - t8853 - t8854 + t8858;
    (t8848, t8853, t8854, t8856, t8858, t8859)
}
