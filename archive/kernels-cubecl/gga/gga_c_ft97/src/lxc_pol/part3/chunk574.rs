//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 574/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk574<F: Float>(t1985: F, t4668: F, t27: F, t89: F, t1008: F, t132: F, t139: F, t1013: F, t3355: F, t2007: F, t4441: F, t4466: F, t528: F) -> (F, F, F, F, F, F, F, F) {
    let t4669 = t1985 * t4668;
    let t4671 = t89 * t27 * t4669;
    let t4673 = t1008 * t1008;
    let t4674 = t4673 * t132;
    let t4675 = t4674 * t139;
    let t4677 = t3355 * t1013;
    let t4680 = t2007 * t4441;
    let t4683 = t528 * t4466;
    (t4669, t4671, t4673, t4674, t4675, t4677, t4680, t4683)
}
