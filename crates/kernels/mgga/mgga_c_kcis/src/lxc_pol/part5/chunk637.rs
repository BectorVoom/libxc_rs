//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 637/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk637<F: Float>(t2938: F, t4690: F, t1670: F, t2960: F, t934: F, t4625: F, t939: F, t1676: F, t659: F) -> (F, F, F, F, F) {
    let t4692 = 2.0 * t2938 * t4690;
    let t4700 = t2960 * t1670;
    let t4701 = t4700 * t934;
    let t4703 = t939 * t4625;
    let t4706 = t659 * t1676;
    (t4692, t4700, t4701, t4703, t4706)
}
