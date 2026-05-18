//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 1002/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk1002<F: Float>(t38688: F, t895: F, t13814: F, t4953: F, t1445: F, t1562: F, t38613: F, t874: F, t40320: F, t13826: F, t1580: F, t46952: F, t568: F, t597: F, t600: F) -> (F, F, F, F, F, F) {
    let t47995 = t895 * t38688;
    let t47997 = t4953 * t13814;
    let t48001 = t1562 * t1445 * t38613 * t874;
    let t48011 = F::new(0.72851559312449424385e1) * t40320;
    let t48013 = F::new(0.23005755572352449806e1) * t1580 * t13826;
    let t48017 = F::new(0.23005755572352449806e1) * t597 * t568 * t600 * t46952;
    (t47995, t47997, t48001, t48011, t48013, t48017)
}
