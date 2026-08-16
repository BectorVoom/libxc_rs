//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 873/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk873(t1897: f64, t35573: f64, t954: f64, t2508: f64, t3009: f64, t32356: f64, t7226: f64, t13560: f64, t169: f64, t1841: f64, t270: f64, t299: f64, t43042: f64, t43090: f64, t43095: f64, t43098: f64, t44866: f64, t44874: f64, t44879: f64, t44883: f64, t44887: f64, t44889: f64, t44895: f64, t44898: f64, t44901: f64, t44905: f64, t44906: f64, t650: f64, t706: f64, t7289: f64, t738: f64, t779: f64, t8867: f64) -> f64 {
    let t44912 = 0.76905262301422242837e-2_f64 * t1897 * t954 * t35573;
    let t44916 = 0.92286314761706691402e-1_f64 * t2508 * t7226 * t3009 * t32356;
    let t44917 = 0.76905262301422242837e-2_f64 * t270 * t706 * t44866 * t169 * t299 - 0.10254034973522965712e-1_f64 * t650 * t13560 - 0.76905262301422242837e-2_f64 * t270 * t738 * t44874 - 0.17090058289204942852e-2_f64 * t1841 * t8867 * t44879 + t44883 - t44887 - 0.34180116578409885704e-2_f64 * t1841 * t7289 * t44889 + 0.3845263115071112142e-2_f64 * t43042 + 0.1281754371690370714e-2_f64 * t43090 + t44895 - 0.34180116578409885707e-2_f64 * t43095 + 0.5127017486761482856e-2_f64 * t43098 + t44898 - t44901 + t44905 + 0.15381052460284448567e-1_f64 * t2508 * t779 * t44906 - t44912 - t44916;
    t44917
}
