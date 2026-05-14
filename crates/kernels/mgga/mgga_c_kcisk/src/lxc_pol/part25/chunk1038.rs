//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1038/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1038<F: Float>(t4998: F, t7610: F, t2013: F, t7614: F, t10886: F, t7605: F, t15921: F, t5486: F, t5006: F, t12169: F, t15930: F, t10832: F, t240: F, t6847: F, t15991: F, t10572: F, t10574: F, t10576: F, t15993: F, t16001: F, t16006: F, t16011: F, t16015: F, t16019: F, t16024: F) -> (F, F, F, F, F, F, F) {
    let t18451 = t4998 * t7610;
    let t18453 = 0.59969295720591057378e-2 * t2013 * t18451;
    let t18454 = t4998 * t7614;
    let t18456 = 0.11993859144118211476e-1 * t2013 * t18454;
    let t18457 = t10886 * t7605;
    let t18458 = t2013 * t18457;
    let t18460 = t5486 * t15921;
    let t18461 = t5006 * t18460;
    let t18464 = t12169 * t15930;
    let t18465 = t10832 * t18464;
    let t18472 = t240 * t6847;
    let t18499 = 0.22954444444444444444e0 * t15991;
    let t18507 = 0.11477222222222222222e0 * t10572 - 0.34431666666666666666e0 * t10574 + 0.17215833333333333333e0 * t10576 + t18499 - 0.68863333333333333333e0 * t15993 - 0.57386111111111111112e0 * t16001 + 0.20659e1 * t16006 + 0.13772666666666666667e1 * t16011 - 0.34431666666666666667e0 * t16015 - 0.309885e1 * t16019 - 0.41318e1 * t16024;
    (t18453, t18456, t18458, t18461, t18465, t18472, t18507)
}
