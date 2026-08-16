//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1018/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1018<F: Float>(t460: F, t491: F, t7286: F, t27453: F, t27721: F, t466: F, t7280: F, t7999: F, t1186: F, t8010: F, t1170: F, t2121: F) -> (F, F, F, F, F) {
    let t27798 = t460 * t491;
    let t27799 = t27798 * t7286;
    let t27800 = t27453 * t27799;
    let t27805 = t466 * t27721;
    let t27808 = t7999 * t7280;
    let t27812 = t1186 * t8010;
    let t27817 = t1170 * t8010;
    let t27818 = t2121 * t27817;
    (t27800, t27805, t27808, t27812, t27818)
}
