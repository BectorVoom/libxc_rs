//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1313/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1313<F: Float>(t30192: F, t30215: F, t3: F, t112: F, t8240: F, t1458: F, t8143: F, t2180: F, t4072: F, t671: F, t8230: F, t12524: F, t1401: F, t16521: F, t16524: F, t20173: F, t29993: F, t29996: F, t30180: F, t3938: F, t3941: F, t5371: F, t5376: F, t577: F, t8161: F, t8166: F, t8251: F) -> (F, F, F, F, F, F, F) {
    let t30217 = F::cast_from(2.0_f64) * t30192 + F::cast_from(2.0_f64) * t30215;
    let t30218 = t3 * t30217;
    let t30231 = t8240 * t112;
    let t30250 = t8143 * t1458;
    let t30253 = t2180 * t4072;
    let t30258 = t8230 * t671;
    let t30263 = F::cast_from(0.45e1_f64) * t30217 * t577 + F::cast_from(0.135e2_f64) * t30231 * t671 + F::cast_from(0.135e2_f64) * t29993 * t1458 + F::cast_from(27.0_f64) * t29996 * t5376 + F::cast_from(0.135e2_f64) * t8161 * t4072 + F::cast_from(0.135e2_f64) * t16521 * t2180 + F::cast_from(27.0_f64) * t16524 * t8166 + F::cast_from(0.135e2_f64) * t5371 * t8143 + F::cast_from(27.0_f64) * t12524 * t8251 + F::cast_from(27.0_f64) * t20173 * t8251 + F::cast_from(27.0_f64) * t3941 * t30250 + F::cast_from(27.0_f64) * t3941 * t30253 + F::cast_from(0.135e2_f64) * t3938 * t8230 + F::cast_from(27.0_f64) * t3941 * t30258 + F::cast_from(0.135e2_f64) * t1401 * t30180;
    (t30217, t30218, t30231, t30250, t30253, t30258, t30263)
}
