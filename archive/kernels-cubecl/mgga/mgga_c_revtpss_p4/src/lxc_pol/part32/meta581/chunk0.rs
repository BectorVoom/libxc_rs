//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1909/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1909<F: Float>(t5722: F, t96576: F, t28780: F, t94890: F, t2435: F, t28825: F, t14079: F, t26265: F, t98108: F, t98128: F, t98130: F, t98144: F) -> (F, F, F, F, F, F, F, F) {
    let t102453 = F::cast_from(0.19514881078765566038e-1_f64) * t96576 * t5722;
    let t102458 = F::cast_from(0.28912093960683998208e-1_f64) * t94890 * t28780;
    let t102462 = t2435 * t28825;
    let t102465 = F::cast_from(0.19514881078765566038e-1_f64) * t26265 * t14079;
    let t102468 = F::cast_from(0.16006300097412701803e-1_f64) * t98108;
    let t102477 = F::cast_from(0.32012600194825403606e-1_f64) * t98128;
    let t102478 = F::cast_from(0.80031500487063509014e-2_f64) * t98130;
    let t102487 = F::cast_from(0.57165357490759649296e-4_f64) * t98144;
    (t102453, t102458, t102462, t102465, t102468, t102477, t102478, t102487)
}
