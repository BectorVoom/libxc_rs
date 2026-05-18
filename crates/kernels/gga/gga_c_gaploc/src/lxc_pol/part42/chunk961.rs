//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 961/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk961<F: Float>(t44578: F, t44579: F, t44580: F, t44618: F, t44622: F, t44624: F, t44627: F, t44630: F, t44633: F, t44635: F, t44638: F, t44642: F, t44644: F, t44658: F, t44662: F, t44665: F, t47036: F, t47042: F) -> F {
    let t49961 = -t44578 + t44579 + t44580 + F::new(0.47425011059460249332e-2) * t47036 + F::new(0.63233348079280332443e-2) * t47042 + t44618 + t44622 + t44624 + t44627 + t44630 - t44633 - t44635 - t44638 + t44642 - t44644 - t44658 - t44662 + t44665;
    t49961
}
