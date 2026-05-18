//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 873/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk873<F: Float>(t1897: F, t35573: F, t954: F, t2508: F, t3009: F, t32356: F, t7226: F, t13560: F, t169: F, t1841: F, t270: F, t299: F, t43042: F, t43090: F, t43095: F, t43098: F, t44866: F, t44874: F, t44879: F, t44883: F, t44887: F, t44889: F, t44895: F, t44898: F, t44901: F, t44905: F, t44906: F, t650: F, t706: F, t7289: F, t738: F, t779: F, t8867: F) -> F {
    let t44912 = F::new(0.76905262301422242837e-2) * t1897 * t954 * t35573;
    let t44916 = F::new(0.92286314761706691402e-1) * t2508 * t7226 * t3009 * t32356;
    let t44917 = F::new(0.76905262301422242837e-2) * t270 * t706 * t44866 * t169 * t299 - F::new(0.10254034973522965712e-1) * t650 * t13560 - F::new(0.76905262301422242837e-2) * t270 * t738 * t44874 - F::new(0.17090058289204942852e-2) * t1841 * t8867 * t44879 + t44883 - t44887 - F::new(0.34180116578409885704e-2) * t1841 * t7289 * t44889 + F::new(0.3845263115071112142e-2) * t43042 + F::new(0.1281754371690370714e-2) * t43090 + t44895 - F::new(0.34180116578409885707e-2) * t43095 + F::new(0.5127017486761482856e-2) * t43098 + t44898 - t44901 + t44905 + F::new(0.15381052460284448567e-1) * t2508 * t779 * t44906 - t44912 - t44916;
    t44917
}
