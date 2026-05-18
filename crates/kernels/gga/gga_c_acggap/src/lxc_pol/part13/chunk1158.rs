//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1158/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1158<F: Float>(t30817: F, t8948: F, t8793: F, t4434: F, t570: F, t1313: F, t30598: F, t721: F, t1322: F, t7859: F, t2041: F, t4632: F) -> (F, F, F, F, F, F) {
    let t35874 = t30817 * t8948;
    let t35875 = F::new(0.25724410870841842184e-2) * t35874;
    let t35876 = t30817 * t8793;
    let t35877 = F::new(0.37737710747524982482e-2) * t35876;
    let t35879 = t570 * t4434;
    let t35882 = t30598 * t1313 * t721;
    let t35885 = t7859 * t1322 * t721;
    let t35887 = t2041 * t4632;
    (t35875, t35877, t35879, t35882, t35885, t35887)
}
