//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 908/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk908<F: Float>(t3116: F, t6535: F, t3139: F, t875: F, t8840: F, t2168: F, t2190: F, t3131: F, t1114: F, t6671: F, t6674: F, t6414: F, t3180: F, t6711: F, t3134: F, t6538: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9007 = t3116 * t6535 / 24.0;
    let t9009 = t3139 * t8840 * t875;
    let t9011 = t2168 * t9009 / 48.0;
    let t9013 = t3139 * t3131 * t2190;
    let t9015 = t2168 * t9013 / 96.0;
    let t9016 = t1114 * t6671;
    let t9018 = t9016 * t6674 / 24.0;
    let t9019 = 7.0 / 288.0 * t6414;
    let t9021 = t6711 * t3180 / 48.0;
    let t9023 = t6538 * t3134 / 96.0;
    (t9007, t9009, t9011, t9013, t9015, t9018, t9019, t9021, t9023)
}
