//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 957/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk957<F: Float>(t14868: F, t291: F, t800: F, t2719: F, t284: F, t1197: F, t4061: F, t4089: F, t1701: F, t3780: F, t1111: F, t1209: F, t14746: F, t14770: F, t14774: F, t14781: F, t14810: F, t2688: F, t2689: F, t2691: F, t2735: F, t285: F, t4094: F, t4099: F, t4104: F, t4113: F, t4114: F, t811: F, t817: F, t820: F) -> F {
    let t14869 = t291 * t14868;
    let t14870 = t800 * t14869;
    let t14872 = t2719 * t284;
    let t14873 = t14872 * t1197;
    let t14875 = t4061 * t4089;
    let t14882 = t1701 * t3780 * t2719;
    let t14887 = F::new(8.0) * t2691 * t4114 * t14770 + F::new(4.0) * t4113 * t14774 * t820 + F::new(2.0) * t4113 * t4114 * t2735 - F::new(4.0) * t2691 * t14781 * t811 + F::cast_from(0.1208182677680765956e1_f64) * t2689 * t1111 - t285 * t817 * t14810 + F::new(2.0) * t14870 + F::new(2.0) * t14873 + F::new(4.0) * t14875 - F::new(2.0) * t2688 * t1209 + F::cast_from(0.60409133884038297798e0_f64) * t4099 * t14746 + F::cast_from(0.1208182677680765956e1_f64) * t4104 * t14882 - F::cast_from(0.1208182677680765956e1_f64) * t4094 * t14882;
    t14887
}
