//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 515/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk515<F: Float>(t2460: F, t2535: F, t2572: F, t2579: F, t1197: F, t1202: F, t1209: F, t1213: F, t153: F, t1540: F, t156: F, t168: F, t2240: F, t2244: F, t2249: F, t2298: F, t2357: F, t2379: F, t242: F, t245: F) -> (F, F) {
    let t2581 = t2460 + t2535 + t2572 + t2579;
    let t2589 = -t1197 + F::new(0.1675256410710088) * t2240 + t1202 - F::new(0.0837628205355044) * t2379 * t242 - F::new(0.1675256410710088) * t2244 - t1209 - t1213 + F::new(0.039794582218349216) * t2249 - F::new(0.011938374665504766) * t168 * t245 * t2581 + t1540 - F::new(1.1389037339096726) * t2298 + F::new(0.42708890021612717) * t153 * t156 * t2357;
    (t2581, t2589)
}
