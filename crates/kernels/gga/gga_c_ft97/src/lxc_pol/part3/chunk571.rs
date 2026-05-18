//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 571/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk571<F: Float>(t86: F, t112: F, t113: F, t4628: F, t4635: F, t5: F, t989: F, t992: F, t1943: F, t920: F, t1017: F, t72: F, t1023: F, t1526: F, t1527: F, t1942: F, t342: F, t343: F) -> (F, F, F, F) {
    let t87 = F::new(10000000.0) <= t86;
    let t4640 = piecewise3::<f64>(t87, F::new(0.0), t5 * t4628 * t113 / F::new(4.0) + t5 * t989 * t992 / F::new(2.0) + t5 * t112 * t4635 / F::new(4.0));
    let t4641 = t1943 * t920;
    let t4645 = t72 * t1017;
    let t4649 = t1023 - t1942 - t1526 * t1527 * t4641 / F::new(12.0) - t342 * t343 * t4645 / F::new(4.0);
    (t4640, t4641, t4645, t4649)
}
