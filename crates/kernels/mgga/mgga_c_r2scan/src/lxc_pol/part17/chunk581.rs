//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 581/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk581<F: Float>(t1577: F, t1592: F, t1619: F, t1622: F, t2133: F, t2231: F, t2651: F, t2675: F, t2683: F, t2685: F, t2689: F, t2693: F, t2696: F, t2728: F, t2732: F, t3064: F, t3068: F, t3073: F, t3077: F, t3081: F, t3087: F, t3092: F, t3108: F, t3116: F, t3227: F, t535: F, t574: F, t948: F) -> F {
    let t3229 = -t1619 - t1622 - F::new(0.27439371595564631661e-1) * t535 * t3064 - F::new(0.43341108700271342816e-1) * t574 * t3068 + F::new(0.54878743191129263322e-1) * t535 * t3073 + F::new(0.86682217400542685632e-1) * t1577 * t3077 - F::new(0.43341108700271342816e-1) * t574 * t3081 - F::new(0.86682217400542685632e-1) * t2651 * t948 - F::new(0.27439371595564631661e-1) * t535 * t3087 + F::new(0.2600466522016280569e0) * t1592 * t3092 + t3108 + F::new(0.25610080155860322884e0) * t2675 - F::new(0.19514881078765566037e-1) * t2683 + F::new(0.54878743191129263322e-2) * t2685 - F::new(0.11643651550782197811e-1) * t2689 - F::new(0.34930954652346593434e-1) * t2693 + F::new(0.86682217400542685632e-1) * t2133 * t3116 - F::new(0.23115257973478049502e0) * t2696 + t2231 + F::new(0.23115257973478049502e0) * t2728 + F::new(0.69345773920434148506e0) * t2732 + t3227;
    t3229
}
