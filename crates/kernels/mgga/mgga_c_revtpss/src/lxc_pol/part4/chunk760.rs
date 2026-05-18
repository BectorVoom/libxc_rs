//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 760/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk760<F: Float>(t1389: F, t2713: F, t3964: F, t1414: F, t3889: F, t828: F, t2668: F, t550: F, t816: F, t1379: F, t1408: F, t2482: F, t27: F) -> (F, F, F, F) {
    let t3967 = F::new(0.45178982497454656791e-5) * t3964 * t2713 * t1389;
    let t3970 = t1414 * t828 * t3889;
    let t3974 = t2668 * t550 * t816;
    let t3976 = F::new(0.13552000749142754193e-3) * t1379 * t3974;
    let t3978 = t2482 * t1408 * t27;
    (t3967, t3970, t3976, t3978)
}
