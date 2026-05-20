//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1376/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1376<F: Float>(t116702: F, t116722: F, t116735: F, t116865: F, t114838: F, t114841: F, t114844: F, t114847: F, t114850: F, t114853: F, t114865: F, t114871: F, t114873: F, t114875: F, t114877: F, t114879: F, t114882: F, t1918: F, t2170: F, t25063: F, t25066: F, t25069: F, t30985: F, t573: F, t6945: F, t6948: F, t8245: F, param_d: F) -> (F, F) {
    let t116867 = t116702 + t116722 + t116735 + t116865;
    let t116876 = t116867 * t573 * param_d + F::new(9.0) * t1918 * t30985 + F::new(6.0) * t2170 * t25063 + F::new(18.0) * t2170 * t25066 + F::new(3.0) * t2170 * t25069 + F::new(18.0) * t6945 * t8245 + F::new(9.0) * t6948 * t8245 + t114838 + t114841 + t114844 + t114847 + t114850 + t114853 + t114865 + t114871 + t114873 + t114875 + t114877 + t114879 + t114882;
    (t116867, t116876)
}
