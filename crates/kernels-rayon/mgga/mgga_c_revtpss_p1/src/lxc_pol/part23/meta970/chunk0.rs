//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3270/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3270(t47125: f64, t47127: f64, t47135: f64, t48324: f64, t47147: f64, t48335: f64, t40076: f64, t40079: f64, t47131: f64, t47138: f64, t47140: f64, t47142: f64, t47152: f64, t48327: f64, t48330: f64, t48332: f64, t48334: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t85989 = 0.48159733137676571078e0_f64 * t47125;
    let t85990 = 0.16265371950452609763e-1_f64 * t47127;
    let t85991 = 0.21687162600603479684e-1_f64 * t47135;
    let t85992 = 0.97592231702715658578e-1_f64 * t48324;
    let t85993 = 0.10254018858216406658e4_f64 * t47147;
    let t85994 = 0.31168546390226634765e3_f64 * t48335;
    let t85995 = t85989 + t85990 + t47131 - t85991 - t47138 - t47140 + t47142 - t85992 - t48327 + t40076 - t40079 - t85993 - t48330 + t48332 + t47152 - t48334 + t85994;
    (t85989, t85990, t85991, t85992, t85993, t85994, t85995)
}
