//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1139/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1139<F: Float>(t1445: F, t47322: F, t807: F, t41411: F, t43986: F, t43989: F, t43991: F, t43993: F, t43994: F, t43997: F, t44002: F, t44005: F, t44010: F, t44012: F) -> F {
    let t47462 = F::cast_from(0.23005755572352449806e1_f64) * t807 * t1445 * t47322;
    let t47463 = F::cast_from(0.51123901271894332903e0_f64) * t41411;
    let t47466 = t43986 - t43989 + F::cast_from(0.14896037479937677779e-1_f64) * t43991 + t47462 - t43993 - t43994 - t47463 - F::cast_from(0.11502877786176224903e2_f64) * t43997 + t44002 + t44005 + t44010 + F::cast_from(0.29792074959875355558e-1_f64) * t44012;
    t47466
}
