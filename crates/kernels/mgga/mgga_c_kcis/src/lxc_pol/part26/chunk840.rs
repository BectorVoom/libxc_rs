//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 840/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk840<F: Float>(t1369: F, t3999: F, t498: F, t110: F, t1939: F, t493: F, t1930: F, t3974: F, t2469: F, t5714: F, t1368: F, t1593: F, t5727: F) -> (F, F, F, F, F, F, F) {
    let t16831 = t1369 * t3999;
    let t16836 = t1369 * t498;
    let t16841 = t110 * t1939;
    let t16842 = t493 * t16841;
    let t16845 = t1930 * t3974 / F::new(54.0);
    let t16848 = t2469 * t1369;
    let t16849 = t16848 * t5714;
    let t16850 = t1368 * t16849;
    let t16852 = t1593 * t5727;
    (t16831, t16836, t16842, t16845, t16848, t16850, t16852)
}
