//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 639/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk639<F: Float>(t1444: F, t617: F, t1606: F, t616: F, t494: F, t1625: F, t1628: F, t1627: F, t632: F, t629: F, t1646: F, t2629: F) -> (F, F, F, F, F, F, F) {
    let t4445 = t617 * t1444;
    let t4455 = F::new(1.0) / t1606 / t616;
    let t4456 = t494 * t4455;
    let t4475 = t1625 * t1628;
    let t4479 = F::new(1.0) / t1627 / t632;
    let t4480 = t629 * t4479;
    let t4510 = t2629 * t1646;
    (t4445, t4455, t4456, t4475, t4479, t4480, t4510)
}
