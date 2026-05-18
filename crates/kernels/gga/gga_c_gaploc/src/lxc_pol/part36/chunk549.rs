//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 549/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk549<F: Float>(t1897: F, t2508: F, t9592: F, t9597: F, t9600: F, t9605: F, t9608: F, t9611: F, t9614: F, t9618: F, t9620: F, t9622: F, t9661: F, t9718: F, t9763: F) -> F {
    let t9765 = F::new(0.30762104920568897134e-1) * t2508 * t9592 + F::new(0.76905262301422242837e-2) * t1897 * t9597 - F::new(0.46143157380853345702e-1) * t2508 * t9600 - F::new(0.15381052460284448567e-1) * t1897 * t9605 - F::new(0.76905262301422242837e-2) * t1897 * t9608 + F::new(0.76905262301422242837e-2) * t2508 * t9611 + F::new(0.23071578690426672851e-1) * t1897 * t9614 + t9618 - t9620 - t9622 + t9661 + t9718 + t9763;
    t9765
}
