//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1126/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1126<F: Float>(t4718: F, t4722: F, t2986: F, t6365: F, t9770: F, t949: F, t9768: F, t3031: F, t6423: F, t4764: F, t13864: F, t4690: F) -> (F, F, F, F) {
    let t18981 = t4722 * t4718;
    let t18983 = F::cast_from(0.32163648644302209644e2_f64) * t2986 * t18981;
    let t18984 = t6365 * t9770;
    let t18985 = t18984 * t949;
    let t18987 = F::cast_from(0.51725014705706168417e3_f64) * t9768 * t18985;
    let t18988 = t3031 * t6423;
    let t18989 = t18988 * t4764;
    let t18993 = F::cast_from(4.0_f64) * t13864 * t4690;
    (t18983, t18987, t18989, t18993)
}
