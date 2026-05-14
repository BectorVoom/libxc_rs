//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1122/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1122<F: Float>(t224: F, t33137: F, t34298: F, t35717: F, t36081: F, t11283: F, t11297: F, t12006: F, t987: F, t33103: F, t33105: F, t33110: F, t33113: F, t33114: F, t33116: F, t33119: F, t33144: F, t33147: F, t34285: F, t34287: F, t34295: F) -> (F, F, F) {
    let t36084 = t224 * (t33137 + t34298 + t35717 + t36081);
    let t36089 = 4.0 * t11283;
    let t36090 = 2.0 * t11297;
    let t38845 = t987 * t12006;
    let t38871 = t33103 - t33105 - t33110 - t33113 - t33114 + t33116 - t33119 + t36084 - t33144 + t33147 - t34285 + t34287 + t34295 + 2.0 * t38845;
    (t36089, t36090, t38871)
}
