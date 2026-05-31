//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1339/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1339<F: Float>(t1916: F, t30191: F, t30194: F, t114401: F, t117: F, t572: F, t114826: F, t114838: F, t114841: F, t114844: F, t114847: F, t114850: F, t114853: F, t114865: F, t114871: F, t114873: F, t114875: F, t1918: F, t2040: F, t25063: F, t25066: F, t25069: F, t30171: F, t573: F, t6945: F, t6948: F, t7944: F, param_d: F) -> F {
    let t114877 = F::cast_from(18.0_f64) * t1916 * t30191;
    let t114879 = F::cast_from(9.0_f64) * t1916 * t30194;
    let t114882 = F::cast_from(3.0_f64) * t572 * t117 * t114401;
    let t114883 = t114826 * t573 * param_d + F::cast_from(9.0_f64) * t1918 * t30171 + F::cast_from(6.0_f64) * t2040 * t25063 + F::cast_from(18.0_f64) * t2040 * t25066 + F::cast_from(3.0_f64) * t2040 * t25069 + F::cast_from(18.0_f64) * t6945 * t7944 + F::cast_from(9.0_f64) * t6948 * t7944 + t114838 + t114841 + t114844 + t114847 + t114850 + t114853 + t114865 + t114871 + t114873 + t114875 + t114877 + t114879 + t114882;
    t114883
}
