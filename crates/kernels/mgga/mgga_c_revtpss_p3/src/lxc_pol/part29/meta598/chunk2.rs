//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2029/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2029<F: Float>(t103553: F, t892: F, t26425: F, t98648: F, t1940: F, t2255: F, t7428: F, t102917: F, t2071: F, t2403: F, t25215: F, t26585: F, t27173: F, t27387: F, t28291: F, t28472: F, t30: F, t4541: F, t7432: F, t8020: F, t98652: F, t98675: F, t98705: F, t98709: F, t98736: F, t98780: F, t98793: F, t99543: F) -> (F, F, F, F) {
    let t103554 = t103553 * t892;
    let t103561 = F::cast_from(6.0_f64) * t26425 * t98648;
    let t103570 = F::cast_from(2.0_f64) * t1940 * t7428 * t2255;
    let t103574 = F::cast_from(3.0_f64) * t2403 * t7428 * t27173 + F::cast_from(3.0_f64) * t4541 * t2071 * t98793 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t26425 * t98652 - t102917 - F::cast_from(6.0_f64) * t28291 * t98675 + t28472 * t98780 - t1940 * t7432 * t98736 / F::cast_from(2.0_f64) - t1940 * t26585 * t27387 + t1940 * t103554 * t30 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t26425 * t98709 + t103561 - t1940 * t7432 * t98705 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) * t2403 * t2071 * t99543 + t103570 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t8020 * t25215;
    (t103554, t103561, t103570, t103574)
}
