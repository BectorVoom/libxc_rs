//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 868/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk868<F: Float>(t12760: F, t2861: F, t1016: F, t4: F, t15: F, t963: F, t12630: F, t12723: F, t12730: F, t12735: F, t12741: F, t12747: F, t12757: F, t180: F, t182: F, t183: F, t2925: F, t3144: F, t3155: F, t3156: F, t3162: F, t3166: F, t456: F, t60: F, t852: F, t983: F, t990: F, t991: F, t995: F) -> (F,) {
    let t12761 = t12760 * t2861;
    let t12762 = t1016 * t4;
    let t12765 = t963 * t15;
    let t12769 = -4.0 * t60 * t12630 - 12.0 * t852 * t2925 - 0.19711288999999999999e-2 * t180 * t182 * t12723 - 0.59133866999999999997e-2 * t990 * t991 * t3162 - 0.11826773399999999999e-1 * t180 * t182 * t12730 + 0.11826773399999999999e-1 * t456 * t12735 + 0.78845155999999999997e-2 * t180 * t983 * t3162 - 0.39422577999999999998e-2 * t12741 * t3156 + 0.29566933499999999998e-2 * t990 * t991 * t3166 + 0.58403819259259259257e-3 * t180 * t12747 * t183 + 0.13140859333333333333e-2 * t180 * t3144 * t995 - 0.39422577999999999999e-2 * t180 * t983 * t3166 - 0.59133866999999999997e-2 * t3155 * t12757 + 0.13140859333333333333e-2 * t12761 * t12762 + 0.21901432222222222225e-3 * t990 * t12765 * t183;
    (t12769,)
}
