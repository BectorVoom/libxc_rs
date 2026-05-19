//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 575/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk575<F: Float>(t10141: F, t2268: F, t2293: F, t2787: F, t2343: F, t3327: F, t6313: F, t2317: F, t2761: F, t6525: F, t2321: F, t8237: F) -> (F, F, F, F, F, F) {
    let t10143 = F::cast_from(0.56910013271352299198e-1_f64) * t2268 * t10141;
    let t10144 = t2787 * t2293;
    let t10145 = t2343 * t10144;
    let t10147 = F::cast_from(0.56910013271352299198e-1_f64) * t2268 * t10145;
    let t10150 = F::cast_from(0.37940008847568199465e-1_f64) * t6313 * t3327;
    let t10160 = t2761 * t2317;
    let t10161 = t6525 * t10160;
    let t10162 = F::cast_from(0.11856252764865062333e-2_f64) * t10161;
    let t10163 = t8237 * t2321;
    (t10143, t10144, t10147, t10150, t10162, t10163)
}
