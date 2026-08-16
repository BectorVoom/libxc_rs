//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2301/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2301<F: Float>(t26447: F, t90607: F, t90787: F, t22751: F, t26397: F, t22892: F, t22893: F, t26396: F, t26384: F, t16018: F, t6637: F, t6888: F, t6968: F) -> (F, F, F, F, F) {
    let t90789 = t90607 * t90787 * t26447;
    let t90791 = t22751 * t26397;
    let t90792 = F::cast_from(0.76763589786250567036e-1_f64) * t90791;
    let t90794 = t22892 * t22893 * t26396;
    let t90795 = F::cast_from(0.16449340668482264365e-1_f64) * t90794;
    let t90797 = t22892 * t22893 * t26384;
    let t90798 = F::cast_from(0.16449340668482264365e-1_f64) * t90797;
    let t90801 = t6888 * t6637 * t6968 * t16018;
    (t90789, t90792, t90795, t90798, t90801)
}
