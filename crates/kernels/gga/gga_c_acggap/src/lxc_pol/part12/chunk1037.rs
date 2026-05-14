//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1037/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1037<F: Float>(t35088: F, t35090: F, t35092: F, t35096: F, t35113: F, t30924: F, t30926: F, t30928: F, t30932: F, t30935: F, t30938: F, t30945: F, t35084: F, t35100: F, t35105: F, t35109: F, t35117: F, t35121: F) -> (F,) {
    let t37379 = 0.42874018118069736972e-3 * t35088;
    let t37380 = 0.11321313224257494745e-1 * t35090;
    let t37381 = 0.37737710747524982482e-2 * t35092;
    let t37382 = 0.42874018118069736972e-2 * t35096;
    let t37386 = 0.18868855373762491241e-1 * t35113;
    let t37396 = -0.42874018118069736972e-3 * t35084 + t37379 + t37380 - t37381 - t37382 - 0.25724410870841842184e-2 * t35100 + 0.42874018118069736972e-3 * t35105 - 0.21437009059034868486e-3 * t35109 + t37386 - 0.18868855373762491241e-1 * t35117 + 0.37737710747524982484e-1 * t35121 + 0.15095084299009992993e-1 * t30924 - 0.22642626448514989489e-1 * t30926 - 0.15095084299009992993e-1 * t30928 - 0.37737710747524982482e-1 * t30932 - 0.2264262644851498949e-1 * t30935 + 0.37737710747524982482e-2 * t30938 + 0.62896184579208304138e-3 * t30945;
    (t37396,)
}
