//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1060/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1060<F: Float>(t11889: F, t2210: F, t858: F, t884: F, t904: F, t933: F, t9807: F, t11874: F, t11876: F, t11880: F, t11885: F, t11888: F, t6506: F, t9041: F, t9086: F, t9096: F, t929: F, t9549: F, t9565: F) -> (F, F, F) {
    let t11891 = t2210 * t858 * t11889;
    let t11893 = t884 * t11891 / F::new(8.0);
    let t11896 = t933 * t904 * t9807;
    let t11899 = t9549 - t11874 + t11876 - t9041 + t11880 + t11885 - t11888 + t11893 - F::new(119.0) / F::new(3456.0) * t6506 + t9086 - t9096 - t929 * t11896 / F::new(768.0) - t9565;
    (t11893, t11896, t11899)
}
