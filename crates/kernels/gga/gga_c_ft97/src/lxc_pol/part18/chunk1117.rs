//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1117/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1117<F: Float>(t1349: F, t24121: F, t376: F, t2001: F, t23773: F, t22642: F, t23827: F, t23825: F, t22511: F, t23823: F, t5818: F, t23704: F, t5838: F, t92385: F, t92388: F, t22708: F, t549: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t94363 = t1349 * t376 * t24121;
    let t94387 = t2001 * t23773;
    let t94394 = t22642 * t23827;
    let t94395 = t23825 * t94394;
    let t94400 = t23823 * t22511;
    let t94401 = t2001 * t94400;
    let t94429 = t5818 * t94400;
    let t94434 = t2001 * t23704;
    let t94443 = t5838 * t92385;
    let t94447 = t5838 * t92388;
    let t94460 = t549 * t22708;
    (t94363, t94387, t94394, t94395, t94400, t94401, t94429, t94434, t94443, t94447, t94460)
}
