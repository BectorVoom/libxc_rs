//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 890/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk890<F: Float>(t3906: F, t898: F, t3898: F, t4442: F, t21780: F, t3739: F, t19804: F, t3912: F, t19637: F, t2242: F, t3893: F, t3724: F, t3903: F, t3889: F, t3916: F, t6792: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t35889 = t3906 * t898;
    let t35929 = t4442 * t3898;
    let t35941 = t21780 * t3739;
    let t36041 = t3912 * t19804;
    let t36114 = t3912 * t19637;
    let t36152 = t2242 * t3893;
    let t36244 = t2242 * t3724;
    let t36246 = t4442 * t3903;
    let t36290 = t2242 * t3889;
    let t36323 = t3916 * t6792;
    (t35889, t35929, t35941, t36041, t36114, t36152, t36244, t36246, t36290, t36323)
}
