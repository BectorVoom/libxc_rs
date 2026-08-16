//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3258/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3258(t22956: f64, t3930: f64, t22886: f64, t9744: f64, t13790: f64, t13845: f64, t13847: f64, t73856: f64, t1353: f64, t13783: f64, t13784: f64, t13789: f64, t13926: f64, t1410: f64, t1872: f64, t1883: f64, t21969: f64, t22809: f64, t22848: f64, t22893: f64, t3934: f64, t3936: f64, t3944: f64, t4012: f64, t5591: f64, t5671: f64, t5689: f64, t6816: f64, t6849: f64, t6862: f64, t74177: f64, t74264: f64, t74269: f64, t74271: f64, t74277: f64, t74279: f64, t74281: f64, t74288: f64, t800: f64, t828: f64, t9748: f64) -> f64 {
    let t85782 = t3930 * t22956;
    let t85791 = t9744 * t22886;
    let t85816 = t13845 * t13847 * t73856 * t13790;
    let t85830 = 0.10003937560882938627e-2_f64 * t85782 + 0.27107389498472794075e-3_f64 * t74264 + 7.0_f64 / 4.0_f64 * t74269 - 7.0_f64 / 8.0_f64 * t74271 + t3944 * t800 * t22848 * t1353 / 16.0_f64 - 7.0_f64 / 16.0_f64 * t85791 - 0.12862205435420921092e-1_f64 * t3934 * t13783 * t13926 * t22893 + 0.42874018118069736972e-2_f64 * t1410 * t4012 * t828 * t22809 * t1353 - 3.0_f64 / 4.0_f64 * t9748 * t800 * t6849 * t5591 + 3.0_f64 / 16.0_f64 * t3944 * t800 * t5689 * t6816 + 3.0_f64 / 16.0_f64 * t3944 * t800 * t1872 * t21969 + 0.7623000421392799234e-4_f64 * t85816 + 0.25724410870841842183e-2_f64 * t3934 * t3936 * t74177 * t1883 - 0.68026775414003982661e-1_f64 * t74277 + 0.34013387707001991332e0_f64 * t74279 - 0.22866142996303859719e-3_f64 * t74281 - 0.1543464652250510531e-1_f64 * t5671 * t13789 * t6862 * t13784 - 0.6098400337114239387e-3_f64 * t74288;
    t85830
}
