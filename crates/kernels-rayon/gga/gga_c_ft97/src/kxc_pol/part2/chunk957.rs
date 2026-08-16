//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 957/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk957(t14868: f64, t291: f64, t800: f64, t2719: f64, t284: f64, t1197: f64, t4061: f64, t4089: f64, t1701: f64, t3780: f64, t1111: f64, t1209: f64, t14746: f64, t14770: f64, t14774: f64, t14781: f64, t14810: f64, t2688: f64, t2689: f64, t2691: f64, t2735: f64, t285: f64, t4094: f64, t4099: f64, t4104: f64, t4113: f64, t4114: f64, t811: f64, t817: f64, t820: f64) -> f64 {
    let t14869 = t291 * t14868;
    let t14870 = t800 * t14869;
    let t14872 = t2719 * t284;
    let t14873 = t14872 * t1197;
    let t14875 = t4061 * t4089;
    let t14882 = t1701 * t3780 * t2719;
    let t14887 = 8.0_f64 * t2691 * t4114 * t14770 + 4.0_f64 * t4113 * t14774 * t820 + 2.0_f64 * t4113 * t4114 * t2735 - 4.0_f64 * t2691 * t14781 * t811 + 0.1208182677680765956e1_f64 * t2689 * t1111 - t285 * t817 * t14810 + 2.0_f64 * t14870 + 2.0_f64 * t14873 + 4.0_f64 * t14875 - 2.0_f64 * t2688 * t1209 + 0.60409133884038297798e0_f64 * t4099 * t14746 + 0.1208182677680765956e1_f64 * t4104 * t14882 - 0.1208182677680765956e1_f64 * t4094 * t14882;
    t14887
}
