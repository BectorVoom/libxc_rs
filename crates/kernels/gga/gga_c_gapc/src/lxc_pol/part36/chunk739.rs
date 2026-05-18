//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 739/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk739<F: Float>(t3698: F, t8785: F, t8784: F, t1672: F, t3142: F, t1462: F, t2993: F, t3120: F, t1036: F, t1699: F, t3144: F, t8620: F) -> (F, F, F, F, F, F, F, F) {
    let t8786 = t3698 * t8785;
    let t8787 = t8784 * t8786;
    let t8788 = t1672 * t3142;
    let t8789 = t1462 * t8788;
    let t8790 = t8787 * t8789;
    let t8792 = t2993 * t3120;
    let t8793 = t1036 * t1699;
    let t8794 = t8792 * t8793;
    let t8796 = t8620 * t3144;
    (t8786, t8787, t8788, t8789, t8790, t8793, t8794, t8796)
}
