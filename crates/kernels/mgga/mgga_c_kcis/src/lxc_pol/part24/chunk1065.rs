//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1065/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1065<F: Float>(t26692: F, t27803: F, t44575: F, t7703: F, t8037: F, t27837: F, t2822: F, t27856: F, t7696: F, t1245: F, t27807: F, t291: F, t27812: F, t283: F, t5168: F, t27876: F) -> (F, F, F, F, F, F, F, F) {
    let t95852 = t26692 * t27803;
    let t95855 = t7703 * t44575 * t8037;
    let t95868 = t2822 * t27837;
    let t95877 = 0.12356481481481481482e-2 * t7696 * t27856;
    let t95890 = t1245 * t291 * t27807;
    let t95892 = 0.12378114784505208333e-4 * t27812 * t95890;
    let t95893 = t5168 * t283;
    let t95903 = t2822 * t27876;
    (t95852, t95855, t95868, t95877, t95890, t95892, t95893, t95903)
}
