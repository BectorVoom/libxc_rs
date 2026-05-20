//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1948/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1948<F: Float>(t101226: F, t2047: F, t7706: F, t95283: F, t26179: F, t28105: F, t28109: F, t101156: F, t101323: F, t2048: F, t25102: F, t25110: F, t25114: F, t25162: F, t26187: F, t28133: F, t28141: F, t28602: F, t28635: F, t6963: F, t7343: F, t7352: F, t7964: F) -> F {
    let t101850 = t2047 * t101226;
    let t101870 = F::new(80.0) / F::new(9.0) * t95283 * t7706;
    let t101872 = F::new(80.0) / F::new(9.0) * t26179 * t28105;
    let t101874 = F::new(80.0) / F::new(9.0) * t26179 * t28109;
    let t101875 = F::new(20.0) / F::new(3.0) * t25162 * t101850 - F::new(4.0) / F::new(3.0) * t101323 * t2048 - F::new(10.0) / F::new(3.0) * t28602 * t25110 - F::new(4.0) / F::new(3.0) * t28141 * t7352 - F::new(10.0) / F::new(3.0) * t26187 * t28133 - F::new(4.0) / F::new(3.0) * t25102 * t7964 - F::new(10.0) / F::new(3.0) * t7343 * t101156 - F::new(4.0) / F::new(3.0) * t6963 * t28635 - F::new(5.0) / F::new(3.0) * t28602 * t25114 + t101870 + t101872 + t101874;
    t101875
}
