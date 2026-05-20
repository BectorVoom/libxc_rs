//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1393/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1393<F: Float>(t1343: F, t1353: F, t1448: F, t1450: F, t198: F, t2522: F, t2562: F, t2569: F, t2579: F, t2587: F, t4139: F, t532: F, t5532: F, t5536: F, t5537: F, t5541: F, t5542: F, t5546: F, t5548: F, t5568: F, t5570: F, t5573: F, t5591: F, t5632: F, t5778: F) -> F {
    let t5782 = t1450 * t198 * t532 * t5778 + F::new(3.0) * t1343 * t198 * t5591 + F::new(3.0) * t1353 * t4139 * t5532 + F::new(6.0) * t1353 * t5536 * t5537 - t1448 * t5541 * t5542 - t2522 - t2562 - t2569 + t2579 + t2587 + t5546 - t5548 + t5568 + t5570 - t5573 - t5632;
    t5782
}
