//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 635/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk635<F: Float>(t1268: F, t2938: F, t898: F, t2946: F, t3738: F, t4967: F, t4971: F, t4975: F, t5242: F, t5245: F, t900: F, t2265: F, t2912: F, t4332: F, t4350: F, t5442: F, t5446: F, t5450: F, t5454: F, t631: F) -> (F, F, F, F, F) {
    let t5457 = t1268 * t1268;
    let t5459 = t898 * t2938 * t5457;
    let t5468 = -F::new(0.117377e0) * t5242 + F::new(0.234754e0) * t5245 + t2946 + F::new(0.9628722222222222222e-1) * t3738 - F::new(0.9628722222222222222e-1) * t4967 + F::new(0.28886166666666666666e0) * t4971 - F::new(0.14443083333333333333e0) * t4975;
    let t5470 = t898 * t900 * t5468;
    let t5473 = -t2912 - F::new(2.0) / F::new(9.0) * t4332 - F::new(2.0) / F::new(3.0) * t4350 + t631 * t5442 / F::new(18.0) - F::new(2.0) / F::new(3.0) * t2265 * t5446 - t631 * t5450 / F::new(3.0) + t631 * t5454 / F::new(6.0) - F::new(3.0) / F::new(2.0) * t631 * t5459 + t631 * t5470 / F::new(2.0);
    (t5457, t5459, t5468, t5470, t5473)
}
