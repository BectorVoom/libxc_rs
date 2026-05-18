//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1148/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1148<F: Float>(t1745: F, t977: F, t1487: F, t6076: F, t997: F, t1096: F, t1165: F, t15626: F, t15628: F, t15633: F, t15639: F, t15653: F, t15667: F, t15671: F, t165: F, t169: F, t171: F, t174: F, t1849: F, t3462: F, t4255: F, t4262: F, t5150: F, t5862: F) -> F {
    let t20650 = t977 * t1745;
    let t20652 = t1487 * t1487;
    let t20666 = t997 * t6076;
    let t20670 = -F::new(0.34299214494455789578e-2) * t15626 + t4255 * t4262 * t1849 * t1096 / F::new(2.0) - F::new(0.40015750243531754508e-2) * t20650 + F::new(0.85748036236139473944e-3) * t165 * t169 * t171 * t20652 * t174 + F::new(0.48018900292238105409e-1) * t15628 + F::new(0.10289764348336736873e-1) * t15633 - F::new(0.17149607247227894789e-2) * t15639 - F::new(0.34299214494455789578e-2) * t3462 * t1165 * t5862 * t5150 + F::new(0.85748036236139473944e-3) * t15653 + F::new(0.80031500487063509015e-2) * t20666 + F::new(7.0) / F::new(72.0) * t15667 + F::new(0.10289764348336736873e-1) * t15671;
    t20670
}
