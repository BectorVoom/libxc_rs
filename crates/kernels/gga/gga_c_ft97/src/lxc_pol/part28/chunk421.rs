//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 421/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk421<F: Float>(t5899: F, t6662: F, t2112: F, t6630: F, t1369: F, t28: F, t586: F, t6615: F, t5916: F, t920: F, t1969: F, t446: F, t1017: F, t5778: F, t89: F, t526: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6663 = t5899 * t6662;
    let t6665 = t2112 * t6630;
    let t6667 = t1369 * t28 * t6665;
    let t6669 = t586 * t6615;
    let t6671 = t1369 * t28 * t6669;
    let t6673 = t5916 * t920;
    let t6674 = t1969 * t6673;
    let t6675 = t446 * t6674;
    let t6677 = t5778 * t1017;
    let t6678 = t28 * t6677;
    let t6679 = t89 * t6678;
    let t6681 = t526 * t6615;
    (t6663, t6665, t6667, t6669, t6671, t6673, t6674, t6675, t6677, t6678, t6679, t6681)
}
