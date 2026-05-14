//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 959/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk959<F: Float>(t43783: F, t43787: F, t43790: F, t43793: F, t43800: F, t43803: F, t43806: F, t43809: F, t43812: F, t43815: F, t43817: F, t47389: F, t43820: F, t43822: F, t43825: F, t43830: F, t43833: F, t43836: F, t43841: F, t43844: F, t43849: F, t43854: F, t43858: F, t43861: F) -> (F, F) {
    let t47394 = -t43783 - 0.25561950635947166451e0 * t47389 - t43787 + t43790 + t43793 + t43800 - t43803 + t43806 - t43809 - 0.14896037479937677779e-1 * t43812 + 0.46011511144704899612e1 * t43815 - 0.14896037479937677779e-1 * t43817;
    let t47399 = t43820 + t43822 - 0.35750489951850426669e0 * t43825 - t43830 + t43833 + 0.1022478025437886658e1 * t43836 + t43841 - 0.25561950635947166451e1 * t43844 - t43849 - 0.7150097990370085334e0 * t43854 - t43858 + t43861;
    (t47394, t47399)
}
