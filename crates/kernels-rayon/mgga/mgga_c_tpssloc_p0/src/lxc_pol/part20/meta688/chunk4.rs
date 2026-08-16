//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2609/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2609(t11791: f64, t5024: f64, t11820: f64, t5002: f64, t11153: f64, t4899: f64, t3540: f64, t4961: f64, t11709: f64, t15640: f64, t11738: f64, t15535: f64, t15553: f64, t3447: f64, t3516: f64, t44965: f64, t44968: f64, t44972: f64, t44976: f64, t44982: f64, t4582: f64, t45971: f64) -> f64 {
    let t52991 = t5024 * t11791;
    let t52992 = t52991 / 1296.0_f64;
    let t52993 = t5002 * t11820;
    let t52994 = t52993 / 4608.0_f64;
    let t52995 = t4899 * t11153;
    let t52999 = t4961 * t3540;
    let t53000 = t52999 / 864.0_f64;
    let t53001 = t11709 * t15640;
    let t53013 = -t52992 - t52994 + t3447 * t52995 * t45971 / 12.0_f64 + t53000 + t53001 / 384.0_f64 + t44965 * t15535 / 1024.0_f64 + t11738 * t4582 * t15553 * t3516 / 1024.0_f64 + t44968 / 3456.0_f64 + t44972 / 6912.0_f64 + t44976 / 3456.0_f64 - t44982 / 1152.0_f64;
    t53013
}
