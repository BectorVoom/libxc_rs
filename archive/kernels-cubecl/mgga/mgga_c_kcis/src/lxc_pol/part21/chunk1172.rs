//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1172/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1172<F: Float>(t187: F, t27986: F, t27988: F, t27989: F, t27991: F, t27992: F, t27993: F, t27995: F, t27996: F, t27998: F, t28001: F, t28004: F, t28007: F, t28008: F, t28011: F, t28072: F, t28256: F, t28297: F) -> F {
    let t28300 = t27986 - t27988 - t27989 + t27991 - t27992 - t27993 + t27995 - t27996 + t27998 - t28001 + t28004 + t28007 - t28008 + t28011 - t28072 + t187 * (t28256 + t28297);
    t28300
}
