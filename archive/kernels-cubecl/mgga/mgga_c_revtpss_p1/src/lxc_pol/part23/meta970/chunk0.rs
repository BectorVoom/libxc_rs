//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3270/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3270<F: Float>(t47125: F, t47127: F, t47135: F, t48324: F, t47147: F, t48335: F, t40076: F, t40079: F, t47131: F, t47138: F, t47140: F, t47142: F, t47152: F, t48327: F, t48330: F, t48332: F, t48334: F) -> (F, F, F, F, F, F, F) {
    let t85989 = F::cast_from(0.48159733137676571078e0_f64) * t47125;
    let t85990 = F::cast_from(0.16265371950452609763e-1_f64) * t47127;
    let t85991 = F::cast_from(0.21687162600603479684e-1_f64) * t47135;
    let t85992 = F::cast_from(0.97592231702715658578e-1_f64) * t48324;
    let t85993 = F::cast_from(0.10254018858216406658e4_f64) * t47147;
    let t85994 = F::cast_from(0.31168546390226634765e3_f64) * t48335;
    let t85995 = t85989 + t85990 + t47131 - t85991 - t47138 - t47140 + t47142 - t85992 - t48327 + t40076 - t40079 - t85993 - t48330 + t48332 + t47152 - t48334 + t85994;
    (t85989, t85990, t85991, t85992, t85993, t85994, t85995)
}
