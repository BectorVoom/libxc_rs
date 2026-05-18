//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 873/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk873<F: Float>(t2268: F, t24139: F, t29975: F, t8124: F, t39695: F, t6520: F, t6525: F, t7888: F, t2326: F, t3394: F, t6514: F, t9074: F) -> (F, F, F, F) {
    let t42637 = F::new(0.68292015925622759036e0) * t2268 * t24139 * t8124 * t29975;
    let t42638 = F::new(0.63233348079280332443e-2) * t39695;
    let t42640 = t6525 * t7888 * t6520;
    let t42641 = F::new(0.71137516589190373998e-2) * t42640;
    let t42644 = t9074 * t6514 * t3394 * t2326;
    (t42637, t42638, t42641, t42644)
}
