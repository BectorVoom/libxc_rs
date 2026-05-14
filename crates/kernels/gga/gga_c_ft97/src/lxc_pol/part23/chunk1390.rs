//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1390/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1390<F: Float>(t126732: F, t126778: F, t126812: F, t126826: F, t126851: F, t126872: F, t126912: F, t126942: F, t126980: F, t127004: F, t127039: F, t127754: F, t127793: F, t127833: F, t127874: F, t127914: F, t871: F) -> (F,) {
    let t127919 = t871 * (t126732 + t126778 + t126812 + t126826 + t126851 + t126872 + t126912 + t126942 + t126980 + t127004 + t127039 + t127754 + t127793 + t127833 + t127874 + t127914);
    (t127919,)
}
