//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 1006/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk1006<F: Float>(t19571: F, t684: F, t2881: F, t312: F, t5299: F, t2874: F, t15195: F, t4151: F, t10514: F, t15271: F, t15273: F, t1901: F, t19535: F, t19539: F, t19543: F, t19547: F, t19551: F, t19555: F, t19559: F, t19565: F, t19568: F, t446: F) -> F {
    let t19572 = t19571 * t684;
    let t19573 = t2881 * t19572;
    let t19576 = t312 * t5299;
    let t19577 = t19576 * t684;
    let t19578 = t2874 * t19577;
    let t19581 = t15195 * t4151;
    let t19584 = F::new(2.0) / F::new(9.0) * t1901 * t19535 - F::new(2.0) / F::new(27.0) * t1901 * t19539 + F::new(2.0) / F::new(27.0) * t1901 * t19543 + F::new(2.0) / F::new(27.0) * t1901 * t19547 + t15271 + t15273 - F::new(2.0) / F::new(3.0) * t446 * t19551 - t446 * t19555 / F::new(3.0) - t446 * t19559 / F::new(3.0) + F::new(4.0) / F::new(27.0) * t10514 + F::new(2.0) / F::new(9.0) * t1901 * t19565 + F::new(2.0) / F::new(9.0) * t1901 * t19568 + t1901 * t19573 / F::new(9.0) + t1901 * t19578 / F::new(9.0) + F::new(2.0) / F::new(9.0) * t1901 * t19581;
    t19584
}
