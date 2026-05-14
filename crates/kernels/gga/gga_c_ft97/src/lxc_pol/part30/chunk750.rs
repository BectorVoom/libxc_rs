//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 750/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk750<F: Float>(t1127: F, t52: F, t7457: F, t11: F, t1690: F, t213: F, t6793: F, t1091: F, t2404: F, t33436: F, t1113: F, t230: F, t420: F, t7470: F, t27729: F, t6: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t35395 = t52 * t7457 * t1127;
    let t35402 = t1690 * t11 * t6793 * t213;
    let t35405 = t2404 * t1091;
    let t35406 = t33436 * t35405;
    let t35409 = t230 * t1113;
    let t35410 = t420 * t35409;
    let t35414 = t230 * t1127;
    let t35415 = t420 * t35414;
    let t35416 = t7470 * t35415;
    let t35419 = t27729 * t6;
    (t35395, t35402, t35405, t35406, t35409, t35410, t35414, t35415, t35416, t35419)
}
