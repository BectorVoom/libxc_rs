//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1326/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1326<F: Float>(t1616: F, t2011: F, t3873: F, t10529: F, t10544: F, t10791: F, t11046: F, t11155: F, t1125: F, t12483: F, t12570: F, t15436: F, t2464: F, t31777: F, t38082: F, t38086: F, t38088: F, t38093: F, t38503: F, t3883: F, t7056: F) -> (F, F, F) {
    let t38508 = F::new(2.0) * t1616 * t3873 * t2011;
    let t38514 = F::new(4.0) * t10529 * t10544;
    let t38515 = F::new(8.0) * t10791 * t11046 + F::new(4.0) * t11046 * t11155 - F::new(2.0) * t1125 * t31777 + F::new(4.0) * t12483 * t7056 - F::new(2.0) * t12570 * t2464 + F::new(2.0) * t15436 * t3883 + t38082 + t38086 + t38088 - t38093 + t38503 - t38508 - t38514;
    (t38508, t38514, t38515)
}
