//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3258/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3258<F: Float>(t22956: F, t3930: F, t22886: F, t9744: F, t13790: F, t13845: F, t13847: F, t73856: F, t1353: F, t13783: F, t13784: F, t13789: F, t13926: F, t1410: F, t1872: F, t1883: F, t21969: F, t22809: F, t22848: F, t22893: F, t3934: F, t3936: F, t3944: F, t4012: F, t5591: F, t5671: F, t5689: F, t6816: F, t6849: F, t6862: F, t74177: F, t74264: F, t74269: F, t74271: F, t74277: F, t74279: F, t74281: F, t74288: F, t800: F, t828: F, t9748: F) -> F {
    let t85782 = t3930 * t22956;
    let t85791 = t9744 * t22886;
    let t85816 = t13845 * t13847 * t73856 * t13790;
    let t85830 = F::cast_from(0.10003937560882938627e-2_f64) * t85782 + F::cast_from(0.27107389498472794075e-3_f64) * t74264 + F::cast_from(7.0_f64) / F::cast_from(4.0_f64) * t74269 - F::cast_from(7.0_f64) / F::cast_from(8.0_f64) * t74271 + t3944 * t800 * t22848 * t1353 / F::cast_from(16.0_f64) - F::cast_from(7.0_f64) / F::cast_from(16.0_f64) * t85791 - F::cast_from(0.12862205435420921092e-1_f64) * t3934 * t13783 * t13926 * t22893 + F::cast_from(0.42874018118069736972e-2_f64) * t1410 * t4012 * t828 * t22809 * t1353 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t9748 * t800 * t6849 * t5591 + F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t3944 * t800 * t5689 * t6816 + F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t3944 * t800 * t1872 * t21969 + F::cast_from(0.7623000421392799234e-4_f64) * t85816 + F::cast_from(0.25724410870841842183e-2_f64) * t3934 * t3936 * t74177 * t1883 - F::cast_from(0.68026775414003982661e-1_f64) * t74277 + F::cast_from(0.34013387707001991332e0_f64) * t74279 - F::cast_from(0.22866142996303859719e-3_f64) * t74281 - F::cast_from(0.1543464652250510531e-1_f64) * t5671 * t13789 * t6862 * t13784 - F::cast_from(0.6098400337114239387e-3_f64) * t74288;
    t85830
}
