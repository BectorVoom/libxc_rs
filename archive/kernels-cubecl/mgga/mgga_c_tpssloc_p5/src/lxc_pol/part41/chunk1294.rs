//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1294/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1294<F: Float>(t103: F, t1419: F, t29900: F, t30530: F, t29895: F, t30510: F, t110333: F, t110601: F, t111121: F, t111134: F, t111331: F, t1453: F, t2349: F, t29903: F, t30063: F, t30297: F, t30303: F, t30307: F, t35656: F, t35663: F, t5480: F, t5488: F, t662: F, t666: F, t8128: F, t8137: F, t8180: F, t8184: F, t96723: F) -> F {
    let t111716 = t1419 * t103;
    let t111720 = t29900 * t30530;
    let t111722 = t29895 * t30510;
    let t111751 = -F::cast_from(25.0_f64) / F::cast_from(27.0_f64) * t8137 * t111716 * t662 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t111720 + F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t111722 - F::cast_from(5.0_f64) / F::cast_from(2.0_f64) * t35656 * t111134 * t30303 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t35663 * t2349 * t1453 * t30307 + t110333 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t110601 * t8184 * t111331 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t29903 * t8180 * t96723 + F::cast_from(5.0_f64) / F::cast_from(12.0_f64) * t8128 * t8184 * t5488 * t662 - F::cast_from(25.0_f64) / F::cast_from(18.0_f64) * t8128 * t30297 * t30303 + F::cast_from(25.0_f64) / F::cast_from(54.0_f64) * t8137 * t111121 * t30307 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t8128 * t30063 * t5480 * t666;
    t111751
}
