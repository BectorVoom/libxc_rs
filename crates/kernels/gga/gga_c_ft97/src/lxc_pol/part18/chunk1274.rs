//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1274/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1274<F: Float>(t23405: F, t26801: F, t358: F, t614: F, t11604: F, t12986: F, t1362: F, t1557: F, t1570: F, t1969: F, t2: F, t23413: F, t24070: F, t24080: F, t24081: F, t24148: F, t26: F, t26785: F, t26809: F, t27420: F, t27421: F, t27426: F, t27427: F, t3188: F, t3424: F, t4: F, t5772: F, t64702: F, t6580: F, t6618: F, t925: F, t94215: F, t94217: F, t94251: F) -> (F,) {
    let t104252 = 2.0 / 27.0 * t23405 * t26801;
    let t104265 = t614 * t358;
    let t104286 = t64702 * t2 * t4 * t26 * t1362 / 6.0 + 2.0 / 9.0 * t94215 - t94217 / 9.0 + t24148 * t6618 / 6.0 - 2.0 / 3.0 * t6580 * t24070 - t104252 + 4.0 / 9.0 * t26809 * t24080 * t24081 * t12986 + 4.0 / 9.0 * t26809 * t27420 * t27421 * t11604 - 4.0 / 27.0 * t26809 * t27426 * t27427 * t11604 + 2.0 / 9.0 * t5772 * t24080 * t104265 * t3424 + 2.0 / 9.0 * t5772 * t27420 * t614 * t1570 * t3188 - 2.0 / 27.0 * t5772 * t27426 * t614 * t1557 * t3188 - t5772 * t1969 * t94251 * t925 / 18.0 - t23413 * t26785 / 9.0;
    (t104286,)
}
